use crate::parser::{Expression, Statement, BinaryOp, UnaryOp};
use crate::types::Value;
use crate::builtins;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        // Try to update existing variable in any scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return;
            }
        }
        // If not found, create in current scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
}

pub struct Evaluator {
    pub env: Environment,
    break_flag: bool,
    continue_flag: bool,
    return_value: Option<Value>,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            break_flag: false,
            continue_flag: false,
            return_value: None,
            recursion_depth: 0,
            max_recursion_depth: 1000, // Prevent stack overflow
        }
    }

    pub fn eval_program(&mut self, statements: &[Statement]) -> Result<()> {
        for stmt in statements {
            self.eval_statement(stmt)?;
            if self.return_value.is_some() {
                break;
            }
        }
        Ok(())
    }

    pub fn eval_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Expression(expr) => {
                let _ = self.eval_expression(expr)?;
                Ok(())
            }
            Statement::VariableDecl { name, value } => {
                let val = self.eval_expression(value)?;
                self.env.set(name.clone(), val);
                Ok(())
            }
            Statement::Assignment { name, value } => {
                let val = self.eval_expression(value)?;
                self.env.set(name.clone(), val);
                Ok(())
            }
            Statement::FunctionDecl { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: Box::new(self.env.clone()),
                };
                
                // Set the function in the environment FIRST
                self.env.set(name.clone(), func.clone());
                
                // Now update the function's closure to include itself for recursion support
                // This is done by creating a new function that captures the current environment
                // (which now includes the function itself)
                let func_with_self = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: Box::new(self.env.clone()),
                };
                self.env.set(name.clone(), func_with_self);
                Ok(())
            }
            Statement::Return(expr) => {
                self.return_value = Some(match expr {
                    Some(e) => self.eval_expression(e)?,
                    None => Value::Null,
                });
                Ok(())
            }
            Statement::If { condition, then_body, else_body } => {
                let cond = self.eval_expression(condition)?;
                if cond.to_bool() {
                    self.env.push_scope();
                    for stmt in then_body {
                        self.eval_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                } else if let Some(else_stmts) = else_body {
                    self.env.push_scope();
                    for stmt in else_stmts {
                        self.eval_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                loop {
                    let cond = self.eval_expression(condition)?;
                    if !cond.to_bool() {
                        break;
                    }

                    self.env.push_scope();
                    for stmt in body {
                        self.eval_statement(stmt)?;
                        if self.return_value.is_some() || self.break_flag || self.continue_flag {
                            break;
                        }
                    }
                    self.env.pop_scope();

                    if self.return_value.is_some() || self.break_flag {
                        break;
                    }
                    self.continue_flag = false;
                }
                self.break_flag = false;
                Ok(())
            }
            Statement::For { var, iter, body } => {
                let iterable = self.eval_expression(iter)?;
                match iterable {
                    Value::Array(arr) => {
                        for item in arr {
                            self.env.set(var.clone(), item);

                            self.env.push_scope();
                            for stmt in body {
                                self.eval_statement(stmt)?;
                                if self.return_value.is_some() || self.break_flag || self.continue_flag {
                                    break;
                                }
                            }
                            self.env.pop_scope();

                            if self.return_value.is_some() || self.break_flag {
                                break;
                            }
                            self.continue_flag = false;
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.env.set(var.clone(), Value::String(ch.to_string()));

                            self.env.push_scope();
                            for stmt in body {
                                self.eval_statement(stmt)?;
                                if self.return_value.is_some() || self.break_flag || self.continue_flag {
                                    break;
                                }
                            }
                            self.env.pop_scope();

                            if self.return_value.is_some() || self.break_flag {
                                break;
                            }
                            self.continue_flag = false;
                        }
                    }
                    _ => return Err(anyhow!("Cannot iterate over {}", iterable.type_name())),
                }
                self.break_flag = false;
                Ok(())
            }
            Statement::Break => {
                self.break_flag = true;
                Ok(())
            }
            Statement::Continue => {
                self.continue_flag = true;
                Ok(())
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Null => Ok(Value::Null),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Identifier(name) => {
                self.env.get(name)
                    .or_else(|| builtins::get_builtin(name))
                    .ok_or_else(|| anyhow!("Undefined variable: '{}'. Check spelling or define it with 'let'.", name))
            }
            Expression::Array(elements) => {
                let mut arr = Vec::new();
                for elem in elements {
                    arr.push(self.eval_expression(elem)?);
                }
                Ok(Value::Array(arr))
            }
            Expression::Object(pairs) => {
                let mut obj = HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.eval_expression(value_expr)?;
                    obj.insert(key.clone(), value);
                }
                Ok(Value::Object(obj))
            }
            Expression::Binary { left, op, right } => {
                let l = self.eval_expression(left)?;
                let r = self.eval_expression(right)?;
                self.eval_binary_op(&l, *op, &r)
            }
            Expression::Unary { op, operand } => {
                let val = self.eval_expression(operand)?;
                self.eval_unary_op(*op, &val)
            }
            Expression::Call { func, args } => {
                let func_val = self.eval_expression(func)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.eval_expression(arg)?);
                }
                self.call_function(func_val, arg_vals)
            }
            Expression::MemberAccess { object, property } => {
                let obj = self.eval_expression(object)?;
                match obj {
                    Value::Object(map) => {
                        Ok(map.get(property).cloned().unwrap_or(Value::Null))
                    }
                    _ => Err(anyhow!("Cannot access property '{}' on {}", property, obj.type_name())),
                }
            }
            Expression::Index { object, index } => {
                let obj = self.eval_expression(object)?;
                let idx = self.eval_expression(index)?;
                match (&obj, &idx) {
                    (Value::Array(arr), Value::Number(n)) => {
                        let index = if *n < 0.0 {
                            // Support negative indexing (Python style)
                            let neg_index = (*n as i32) as i64;
                            let len = arr.len() as i64;
                            ((len + neg_index) as usize)
                        } else {
                            *n as usize
                        };
                        Ok(arr.get(index).cloned().unwrap_or(Value::Null))
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let chars: Vec<char> = s.chars().collect();
                        let index = if *n < 0.0 {
                            let neg_index = (*n as i32) as i64;
                            let len = chars.len() as i64;
                            ((len + neg_index) as usize)
                        } else {
                            *n as usize
                        };
                        let ch = chars.get(index);
                        Ok(ch.map(|c| Value::String(c.to_string())).unwrap_or(Value::Null))
                    }
                    (Value::Object(map), Value::String(key)) => {
                        Ok(map.get(key).cloned().unwrap_or(Value::Null))
                    }
                    _ => Err(anyhow!("Cannot index {} with {}", obj.type_name(), idx.type_name())),
                }
            }
            Expression::Pipe { left, right } => {
                let left_val = self.eval_expression(left)?;
                match right.as_ref() {
                    Expression::Call { func, args } => {
                        let func_val = self.eval_expression(func)?;
                        
                        let mut arg_vals = vec![left_val];
                        for arg in args {
                            arg_vals.push(self.eval_expression(arg)?);
                        }
                        
                        self.call_function(func_val, arg_vals)
                    }
                    Expression::Identifier(name) => {
                        let func_val = self.env.get(name)
                            .or_else(|| builtins::get_builtin(name))
                            .ok_or_else(|| anyhow!("Undefined: {}", name))?;
                        self.call_function(func_val, vec![left_val])
                    }
                    _ => Err(anyhow!("Invalid pipe target")),
                }
            }
            Expression::Lambda { params, body } => {
                Ok(Value::Function {
                    params: params.clone(),
                    body: vec![Statement::Return(Some((**body).clone()))],
                    closure: Box::new(self.env.clone()),
                })
            }
            Expression::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expression(condition)?;
                if cond.to_bool() {
                    self.eval_expression(then_branch)
                } else if let Some(else_expr) = else_branch {
                    self.eval_expression(else_expr)
                } else {
                    Ok(Value::Null)
                }
            }
            Expression::Match { expr, arms } => {
                let val = self.eval_expression(expr)?;
                let val_str = val.to_string_value();
                
                for (pattern, body_expr) in arms {
                    if pattern == &val_str {
                        return self.eval_expression(body_expr);
                    }
                }
                
                Ok(Value::Null)
            }
        }
    }

    fn eval_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::Array(a), Value::Array(b)) => {
                    let mut result = a.clone();
                    result.extend(b.clone());
                    Ok(Value::Array(result))
                }
                _ => Err(anyhow!("Cannot add {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Subtract => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(anyhow!("Cannot subtract {} from {}", right.type_name(), left.type_name())),
            },
            BinaryOp::Multiply => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
                _ => Err(anyhow!("Cannot multiply {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Divide => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(anyhow!("Cannot divide {} by {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Modulo => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(anyhow!("Modulo by zero"))
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(anyhow!("Cannot modulo {} by {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Power => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
                _ => Err(anyhow!("Cannot raise {} to power {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Equal => Ok(Value::Bool(left == right)),
            BinaryOp::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOp::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err(anyhow!("Cannot compare {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err(anyhow!("Cannot compare {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err(anyhow!("Cannot compare {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(anyhow!("Cannot compare {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::And => Ok(Value::Bool(left.to_bool() && right.to_bool())),
            BinaryOp::Or => Ok(Value::Bool(left.to_bool() || right.to_bool())),
        }
    }

    fn eval_unary_op(&self, op: UnaryOp, val: &Value) -> Result<Value> {
        match op {
            UnaryOp::Not => Ok(Value::Bool(!val.to_bool())),
            UnaryOp::Negate => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(anyhow!("Cannot negate {}", val.type_name())),
            },
        }
    }

    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value> {
        // Check if this is a builtin
        if builtins::is_builtin(&func) {
            if let Some(name) = builtins::get_builtin_name(&func) {
                return builtins::call_builtin(&name, args);
            }
        }

        match func {
            Value::Function { params, body, closure } => {
                // Check recursion depth
                if self.recursion_depth >= self.max_recursion_depth {
                    return Err(anyhow!(
                        "Maximum recursion depth ({}) exceeded. Check for infinite loops.",
                        self.max_recursion_depth
                    ));
                }

                if params.len() != args.len() {
                    return Err(anyhow!("Expected {} arguments, got {}", params.len(), args.len()));
                }

                let saved_env = self.env.clone();
                
                // For recursion support: use the closure as base, but also keep
                // global functions accessible by merging with saved environment
                self.env = (*closure).clone();
                
                // Merge in any global functions from the saved environment (for recursion)
                for scope in saved_env.scopes.iter() {
                    for (name, value) in scope {
                        if matches!(value, Value::Function { .. }) && self.env.get(name).is_none() {
                            self.env.set(name.clone(), value.clone());
                        }
                    }
                }
                
                self.env.push_scope();

                for (param, arg) in params.iter().zip(args.iter()) {
                    self.env.set(param.clone(), arg.clone());
                }

                self.recursion_depth += 1;
                let ret_val = self.return_value.take();
                for stmt in &body {
                    self.eval_statement(stmt)?;
                    if self.return_value.is_some() {
                        break;
                    }
                }

                let result = self.return_value.take().unwrap_or(Value::Null);
                self.return_value = ret_val;
                self.recursion_depth -= 1;
                self.env = saved_env;

                Ok(result)
            }
            _ => Err(anyhow!("Not a function")),
        }
    }
}
