use crate::types::Value;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;


pub fn get_builtin(name: &str) -> Option<Value> {
    match name {
        "print" | "echo" | "println" | "log" | "len" | "type" | "keys" | "values" |
        "push" | "pop" | "shift" | "unshift" | "join" | "split" | "trim" | "upper" |
        "lower" | "reverse" | "sort" | "find" | "index_of" | "each" | "map" | "filter" |
        "reduce" | "range" | "sum" | "avg" | "min" | "max" | "abs" | "floor" | "ceil" |
        "round" | "sqrt" | "pow" | "contains" | "starts_with" | "ends_with" | "replace" |
        "to_string" | "to_number" | "read_file" | "write_file" | "exists" | "ls" |
        "slice" | "unique" | "flatten" | "compact" | "includes?" | "empty?" | "any?" |
        "all?" | "chars" | "lines" | "capitalize" | "indent" | "pad_left" | "pad_right" |
        "debug" | "error" => {
            // Return a special marker that indicates this is a builtin
            Some(Value::String(format!("__builtin_{}", name)))
        }
        _ => None,
    }
}

pub fn is_builtin(val: &Value) -> bool {
    match val {
        Value::String(s) => s.starts_with("__builtin_"),
        _ => false,
    }
}

pub fn get_builtin_name(val: &Value) -> Option<String> {
    match val {
        Value::String(s) if s.starts_with("__builtin_") => {
            Some(s.strip_prefix("__builtin_").unwrap().to_string())
        }
        _ => None,
    }
}

pub fn call_builtin(name: &str, args: Vec<Value>) -> Result<Value> {
    match name {
        "print" | "echo" | "println" | "log" => builtin_print(args),
        "len" => builtin_len(args),
        "type" => builtin_type(args),
        "keys" => builtin_keys(args),
        "values" => builtin_values(args),
        "range" => builtin_range(args),
        "join" => builtin_join(args),
        "split" => builtin_split(args),
        "trim" => builtin_trim(args),
        "upper" => builtin_upper(args),
        "lower" => builtin_lower(args),
        "reverse" => builtin_reverse(args),
        "contains" => builtin_contains(args),
        "starts_with" => builtin_starts_with(args),
        "ends_with" => builtin_ends_with(args),
        "replace" => builtin_replace(args),
        "to_string" => builtin_to_string(args),
        "to_number" => builtin_to_number(args),
        "sum" => builtin_sum(args),
        "min" => builtin_min(args),
        "max" => builtin_max(args),
        "avg" => builtin_avg(args),
        "abs" => builtin_abs(args),
        "floor" => builtin_floor(args),
        "ceil" => builtin_ceil(args),
        "round" => builtin_round(args),
        "sqrt" => builtin_sqrt(args),
        "pow" => builtin_pow(args),
        "read_file" => builtin_read_file(args),
        "write_file" => builtin_write_file(args),
        "exists" => builtin_exists(args),
        "ls" => builtin_ls(args),
        "push" => builtin_push(args),
        "pop" => builtin_pop(args),
        "shift" => builtin_shift(args),
        "unshift" => builtin_unshift(args),
        "find" => builtin_find(args),
        "index_of" => builtin_index_of(args),
        "slice" => builtin_slice(args),
        "sort" => builtin_sort(args),
        "unique" => builtin_unique(args),
        "flatten" => builtin_flatten(args),
        "compact" => builtin_compact(args),
        "includes?" => builtin_includes(args),
        "empty?" => builtin_empty(args),
        "any?" => builtin_any(args),
        "all?" => builtin_all(args),
        "chars" => builtin_chars(args),
        "lines" => builtin_lines(args),
        "capitalize" => builtin_capitalize(args),
        "indent" => builtin_indent(args),
        "pad_left" => builtin_pad_left(args),
        "pad_right" => builtin_pad_right(args),
        "debug" => builtin_debug(args),
        "error" => builtin_error(args),
        _ => Err(anyhow!("Unknown builtin: {}", name)),
    }
}

pub fn builtin_print(args: Vec<Value>) -> Result<Value> {
    for arg in args {
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
    
    // Security: Only allow reading from current directory or subdirectories
    let canonical_path = std::fs::canonicalize(&path)?;
    let current_dir = std::env::current_dir()?;
    
    if !canonical_path.starts_with(&current_dir) {
        return Err(anyhow!("Security: Cannot read files outside current directory"));
    }
    
    let content = fs::read_to_string(&path)?;
    Ok(Value::String(content))
}

pub fn builtin_write_file(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("write_file() requires 2 arguments"));
    }
    
    let path = args[0].to_string_value();
    let content = args[1].to_string_value();
    
    // Security: Only allow writing to current directory or subdirectories
    let canonical_path = std::fs::canonicalize(&path).unwrap_or_else(|_| std::path::PathBuf::from(&path));
    let current_dir = std::env::current_dir()?;
    
    // Check if the parent directory is within current dir
    if let Some(parent) = canonical_path.parent() {
        if !parent.starts_with(&current_dir) && parent != current_dir {
            return Err(anyhow!("Security: Cannot write files outside current directory"));
        }
    }
    
    fs::write(&path, content)?;
    Ok(Value::Null)
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

// Array mutation operations
pub fn builtin_push(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("push() requires at least 2 arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err(anyhow!("push() requires an array")),
    }
}

pub fn builtin_pop(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("pop() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        }
        _ => Err(anyhow!("pop() requires an array")),
    }
}

pub fn builtin_shift(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("shift() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(arr[0].clone())
            }
        }
        _ => Err(anyhow!("shift() requires an array")),
    }
}

pub fn builtin_unshift(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("unshift() requires at least 2 arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = vec![args[1].clone()];
            new_arr.extend(arr.clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err(anyhow!("unshift() requires an array")),
    }
}

pub fn builtin_find(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("find() requires 2 arguments (array, predicate)"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            // Simple find by equality for now
            for item in arr {
                if item == &args[1] {
                    return Ok(item.clone());
                }
            }
            Ok(Value::Null)
        }
        _ => Err(anyhow!("find() requires an array")),
    }
}

pub fn builtin_index_of(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("index_of() requires 2 arguments"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                if item == &args[1] {
                    return Ok(Value::Number(i as f64));
                }
            }
            Ok(Value::Number(-1.0))
        }
        Value::String(s) => {
            if let Value::String(search) = &args[1] {
                match s.find(search) {
                    Some(pos) => Ok(Value::Number(pos as f64)),
                    None => Ok(Value::Number(-1.0)),
                }
            } else {
                Err(anyhow!("index_of() on string requires string search term"))
            }
        }
        _ => Err(anyhow!("index_of() requires an array or string")),
    }
}

pub fn builtin_slice(args: Vec<Value>) -> Result<Value> {
    if args.len() < 3 {
        return Err(anyhow!("slice() requires 3 arguments (array, start, end)"));
    }
    
    let start = args[1].to_number().ok_or_else(|| anyhow!("slice() requires numeric start"))?;
    let end = args[2].to_number().ok_or_else(|| anyhow!("slice() requires numeric end"))?;
    let start_idx = start as usize;
    let end_idx = end as usize;
    
    match &args[0] {
        Value::Array(arr) => {
            let sliced = arr.iter()
                .skip(start_idx)
                .take(end_idx.saturating_sub(start_idx))
                .cloned()
                .collect();
            Ok(Value::Array(sliced))
        }
        Value::String(s) => {
            let chars: Vec<char> = s.chars().collect();
            let sliced: String = chars.iter()
                .skip(start_idx)
                .take(end_idx.saturating_sub(start_idx))
                .collect();
            Ok(Value::String(sliced))
        }
        _ => Err(anyhow!("slice() requires an array or string")),
    }
}

pub fn builtin_sort(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("sort() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut sorted = arr.clone();
            sorted.sort_by(|a, b| {
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Value::String(x), Value::String(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            Ok(Value::Array(sorted))
        }
        _ => Err(anyhow!("sort() requires an array")),
    }
}

pub fn builtin_unique(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("unique() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let mut unique = Vec::new();
            for item in arr {
                if !unique.contains(item) {
                    unique.push(item.clone());
                }
            }
            Ok(Value::Array(unique))
        }
        _ => Err(anyhow!("unique() requires an array")),
    }
}

pub fn builtin_flatten(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("flatten() requires 1 argument"));
    }
    
    fn flatten_inner(val: &Value) -> Vec<Value> {
        match val {
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    result.extend(flatten_inner(item));
                }
                result
            }
            other => vec![other.clone()],
        }
    }
    
    match &args[0] {
        Value::Array(_) => Ok(Value::Array(flatten_inner(&args[0]))),
        _ => Err(anyhow!("flatten() requires an array")),
    }
}

pub fn builtin_compact(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("compact() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            let compacted: Vec<Value> = arr.iter()
                .filter(|v| !matches!(v, Value::Null))
                .cloned()
                .collect();
            Ok(Value::Array(compacted))
        }
        _ => Err(anyhow!("compact() requires an array")),
    }
}

pub fn builtin_avg(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("avg() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(Value::Null);
            }
            let mut sum = 0.0;
            for val in arr {
                if let Some(n) = val.to_number() {
                    sum += n;
                } else {
                    return Err(anyhow!("Cannot average non-numeric array"));
                }
            }
            Ok(Value::Number(sum / arr.len() as f64))
        }
        _ => Err(anyhow!("avg() requires an array")),
    }
}

pub fn builtin_includes(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("includes?() requires 2 arguments"));
    }
    
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(search)) => {
            Ok(Value::Bool(s.contains(search)))
        }
        (Value::Array(arr), item) => {
            Ok(Value::Bool(arr.contains(item)))
        }
        _ => Ok(Value::Bool(false)),
    }
}

pub fn builtin_empty(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("empty?() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => Ok(Value::Bool(arr.is_empty())),
        Value::String(s) => Ok(Value::Bool(s.is_empty())),
        Value::Object(obj) => Ok(Value::Bool(obj.is_empty())),
        Value::Null => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

pub fn builtin_any(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("any?() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            Ok(Value::Bool(arr.iter().any(|v| v.to_bool())))
        }
        _ => Err(anyhow!("any?() requires an array")),
    }
}

pub fn builtin_all(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("all?() requires 1 argument"));
    }
    
    match &args[0] {
        Value::Array(arr) => {
            Ok(Value::Bool(arr.iter().all(|v| v.to_bool())))
        }
        _ => Err(anyhow!("all?() requires an array")),
    }
}

// String operations
pub fn builtin_chars(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("chars() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let chars: Vec<Value> = s.chars()
                .map(|c| Value::String(c.to_string()))
                .collect();
            Ok(Value::Array(chars))
        }
        _ => Err(anyhow!("chars() requires a string")),
    }
}

pub fn builtin_lines(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("lines() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let lines: Vec<Value> = s.lines()
                .map(|line| Value::String(line.to_string()))
                .collect();
            Ok(Value::Array(lines))
        }
        _ => Err(anyhow!("lines() requires a string")),
    }
}

pub fn builtin_capitalize(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("capitalize() requires 1 argument"));
    }
    
    match &args[0] {
        Value::String(s) => {
            let mut chars = s.chars();
            match chars.next() {
                None => Ok(Value::String(String::new())),
                Some(first) => {
                    let capitalized = first.to_uppercase().to_string() + &chars.as_str().to_lowercase();
                    Ok(Value::String(capitalized))
                }
            }
        }
        _ => Err(anyhow!("capitalize() requires a string")),
    }
}

pub fn builtin_indent(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("indent() requires 2 arguments (string, amount)"));
    }
    
    match (&args[0], args[1].to_number()) {
        (Value::String(s), Some(amount)) => {
            let spaces = " ".repeat(amount as usize);
            let indented = s.lines()
                .map(|line| format!("{}{}", spaces, line))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(Value::String(indented))
        }
        _ => Err(anyhow!("indent() requires a string and number")),
    }
}

pub fn builtin_pad_left(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("pad_left() requires 2 arguments (string, length)"));
    }
    
    match (&args[0], args[1].to_number()) {
        (Value::String(s), Some(len)) => {
            let target_len = len as usize;
            if s.len() >= target_len {
                Ok(Value::String(s.clone()))
            } else {
                let padding = " ".repeat(target_len - s.len());
                Ok(Value::String(format!("{}{}", padding, s)))
            }
        }
        _ => Err(anyhow!("pad_left() requires a string and number")),
    }
}

pub fn builtin_pad_right(args: Vec<Value>) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("pad_right() requires 2 arguments (string, length)"));
    }
    
    match (&args[0], args[1].to_number()) {
        (Value::String(s), Some(len)) => {
            let target_len = len as usize;
            if s.len() >= target_len {
                Ok(Value::String(s.clone()))
            } else {
                let padding = " ".repeat(target_len - s.len());
                Ok(Value::String(format!("{}{}", s, padding)))
            }
        }
        _ => Err(anyhow!("pad_right() requires a string and number")),
    }
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
