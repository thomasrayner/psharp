use crate::types::Value;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_builtin(name: &str) -> Option<Value> {
    match name {
        "print" | "echo" | "println" | "log" | "len" | "type" | "keys" | "values" |
        "push" | "pop" | "shift" | "unshift" | "join" | "split" | "trim" | "upper" |
        "lower" | "reverse" | "sort" | "map" | "filter" | "reduce" | "find" | "each" |
        "range" | "sum" | "avg" | "min" | "max" | "abs" | "floor" | "ceil" | "round" |
        "sqrt" | "pow" | "contains" | "starts_with" | "ends_with" | "replace" | "to_string" |
        "to_number" | "read_file" | "write_file" | "exec" | "exists" | "mkdir" | "rm" |
        "ls" | "pwd" | "cd" | "now" | "sleep" | "error" | "assert" | "debug" => {
            Some(Value::Function {
                params: vec![],
                body: vec![],
                closure: Box::new(crate::evaluator::Environment::new()),
            })
        }
        _ => None,
    }
}

pub fn call_builtin(_func: &Value, _args: Vec<Value>) -> Result<Value> {
    Err(anyhow!("Builtin function dispatch not yet implemented"))
}

pub fn builtin_print(args: Vec<Value>) -> Result<Value> {
    for arg in args {
        print!("{}", arg);
    }
    println!();
    Ok(Value::Null)
}

pub fn builtin_echo(args: Vec<Value>) -> Result<Value> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    Ok(Value::Null)
}

pub fn builtin_len(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("len() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
        Value::Object(obj) => Ok(Value::Number(obj.len() as f64)),
        _ => Err(anyhow!("Cannot get length of {}", args[0].type_name())),
    }
}

pub fn builtin_type(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("type() requires 1 argument"));
    }
    Ok(Value::String(args[0].type_name().to_string()))
}

pub fn builtin_keys(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("keys() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Object(obj) => {
            let keys: Vec<Value> = obj.keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::Array(keys))
        }
        _ => Err(anyhow!("Cannot get keys of {}", args[0].type_name())),
    }
}

pub fn builtin_values(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("values() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Object(obj) => {
            let values: Vec<Value> = obj.values().cloned().collect();
            Ok(Value::Array(values))
        }
        _ => Err(anyhow!("Cannot get values of {}", args[0].type_name())),
    }
}

pub fn builtin_range(args: Vec<Value>) -> Result<Value> {
    let (start, end) = match args.len() {
        1 => (0.0, args[0].to_number().ok_or_else(|| anyhow!("range() requires numbers"))?),
        2 => (
            args[0].to_number().ok_or_else(|| anyhow!("range() requires numbers"))?,
            args[1].to_number().ok_or_else(|| anyhow!("range() requires numbers"))?
        ),
        _ => return Err(anyhow!("range() requires 1 or 2 arguments")),
    };

    let mut result = Vec::new();
    let mut i = start;
    while i < end {
        result.push(Value::Number(i));
        i += 1.0;
    }
    Ok(Value::Array(result))
}

pub fn builtin_join(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("join() requires 2 arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let sep = args[1].to_string_value();
            let joined = arr.iter()
                .map(|v| v.to_string_value())
                .collect::<Vec<_>>()
                .join(&sep);
            Ok(Value::String(joined))
        }
        _ => Err(anyhow!("join() requires an array")),
    }
}

pub fn builtin_split(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("split() requires 2 arguments"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let sep = args[1].to_string_value();
            let parts: Vec<Value> = s.split(&sep)
                .map(|p| Value::String(p.to_string()))
                .collect();
            Ok(Value::Array(parts))
        }
        _ => Err(anyhow!("split() requires a string")),
    }
}

pub fn builtin_trim(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("trim() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(anyhow!("trim() requires a string")),
    }
}

pub fn builtin_upper(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("upper() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(anyhow!("upper() requires a string")),
    }
}

pub fn builtin_lower(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("lower() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(anyhow!("lower() requires a string")),
    }
}

pub fn builtin_reverse(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("reverse() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut rev = arr.clone();
            rev.reverse();
            Ok(Value::Array(rev))
        }
        Value::String(s) => {
            let rev: String = s.chars().rev().collect();
            Ok(Value::String(rev))
        }
        _ => Err(anyhow!("reverse() requires an array or string")),
    }
}

pub fn builtin_contains(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("contains() requires 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(search)) => {
            Ok(Value::Bool(s.contains(search)))
        }
        (Value::Array(arr), item) => {
            Ok(Value::Bool(arr.contains(item)))
        }
        _ => Err(anyhow!("contains() invalid argument types")),
    }
}

pub fn builtin_starts_with(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("starts_with() requires 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(prefix)) => {
            Ok(Value::Bool(s.starts_with(prefix)))
        }
        _ => Err(anyhow!("starts_with() requires strings")),
    }
}

pub fn builtin_ends_with(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("ends_with() requires 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(suffix)) => {
            Ok(Value::Bool(s.ends_with(suffix)))
        }
        _ => Err(anyhow!("ends_with() requires strings")),
    }
}

pub fn builtin_replace(args: Vec<Value>) -> Result<Value> {
    if args.len() < 3 {
        return Err(anyhow!("replace() requires 3 arguments"));
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::String(from), Value::String(to)) => {
            Ok(Value::String(s.replace(from, to)))
        }
        _ => Err(anyhow!("replace() requires strings")),
    }
}

pub fn builtin_to_string(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("to_string() requires 1 argument"));
    }
    Ok(Value::String(args[0].to_string_value()))
}

pub fn builtin_to_number(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("to_number() requires 1 argument"));
    }
    
    match args[0].to_number() {
        Some(n) => Ok(Value::Number(n)),
        None => Err(anyhow!("Cannot convert {} to number", args[0].type_name())),
    }
}

pub fn builtin_sum(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("sum() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut total = 0.0;
            for val in arr {
                if let Some(n) = val.to_number() {
                    total += n;
                } else {
                    return Err(anyhow!("Cannot sum non-numeric array"));
                }
            }
            Ok(Value::Number(total))
        }
        _ => Err(anyhow!("sum() requires an array")),
    }
}

pub fn builtin_min(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("min() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut min = f64::INFINITY;
            for val in arr {
                if let Some(n) = val.to_number() {
                    if n < min {
                        min = n;
                    }
                } else {
                    return Err(anyhow!("Cannot find min of non-numeric array"));
                }
            }
            if min == f64::INFINITY {
                Ok(Value::Null)
            } else {
                Ok(Value::Number(min))
            }
        }
        _ => Err(anyhow!("min() requires an array")),
    }
}

pub fn builtin_max(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("max() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut max = f64::NEG_INFINITY;
            for val in arr {
                if let Some(n) = val.to_number() {
                    if n > max {
                        max = n;
                    }
                } else {
                    return Err(anyhow!("Cannot find max of non-numeric array"));
                }
            }
            if max == f64::NEG_INFINITY {
                Ok(Value::Null)
            } else {
                Ok(Value::Number(max))
            }
        }
        _ => Err(anyhow!("max() requires an array")),
    }
}

pub fn builtin_abs(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("abs() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(anyhow!("abs() requires a number")),
    }
}

pub fn builtin_floor(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("floor() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(anyhow!("floor() requires a number")),
    }
}

pub fn builtin_ceil(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("ceil() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(anyhow!("ceil() requires a number")),
    }
}

pub fn builtin_round(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("round() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(anyhow!("round() requires a number")),
    }
}

pub fn builtin_sqrt(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("sqrt() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sqrt())),
        _ => Err(anyhow!("sqrt() requires a number")),
    }
}

pub fn builtin_pow(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("pow() requires 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
        _ => Err(anyhow!("pow() requires numbers")),
    }
}

pub fn builtin_read_file(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("read_file() requires 1 argument"));
    }
    
    let path = args[0].to_string_value();
    let content = fs::read_to_string(&path)?;
    Ok(Value::String(content))
}

pub fn builtin_write_file(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("write_file() requires 2 arguments"));
    }
    
    let path = args[0].to_string_value();
    let content = args[1].to_string_value();
    fs::write(&path, content)?;
    Ok(Value::Null)
}

pub fn builtin_exec(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("exec() requires 1 argument"));
    }
    
    let cmd = args[0].to_string_value();
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(Value::String(stdout.trim().to_string()))
}

pub fn builtin_exists(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("exists() requires 1 argument"));
    }
    
    let path = args[0].to_string_value();
    Ok(Value::Bool(Path::new(&path).exists()))
}

pub fn builtin_ls(args: Vec<Value>) -> Result<Value> {
    let path = if args.is_empty() {
        ".".to_string()
    } else {
        args[0].to_string_value()
    };
    
    let mut entries = Vec::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        entries.push(Value::String(
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        ));
    }
    Ok(Value::Array(entries))
}

pub fn builtin_debug(args: Vec<Value>) -> Result<Value> {
    eprintln!("DEBUG: {:?}", args);
    Ok(Value::Null)
}

pub fn builtin_error(args: Vec<Value>) -> Result<Value> {
    let msg = if args.is_empty() {
        "error".to_string()
    } else {
        args[0].to_string_value()
    };
    Err(anyhow!(msg))
}
