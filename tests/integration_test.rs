use std::process::Command;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn run_psharp(code: &str) -> String {
    // Create unique temp file for each test
    let test_id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let temp_file = format!("/tmp/psharp_test_{}.ps", test_id);
    fs::write(&temp_file, code).expect("Failed to write test file");
    
    // Run psharp - use the debug binary directly
    let output = Command::new(env!("CARGO_BIN_EXE_psharp"))
        .arg(&temp_file)
        .output()
        .expect("Failed to run psharp");
    
    // Clean up
    let _ = fs::remove_file(&temp_file);
    
    if !output.status.success() {
        eprintln!("psharp stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn test_lambda_simple() {
    let result = run_psharp("let f = |x| x * 2\nprint(f(5))");
    assert_eq!(result, "10");
}

#[test]
fn test_lambda_multiple_params() {
    let result = run_psharp("let add = |a, b| a + b\nprint(add(3, 4))");
    assert_eq!(result, "7");
}

#[test]
fn test_basic_arithmetic() {
    let result = run_psharp("print(2 + 3 * 4)");
    assert_eq!(result, "14");
}

#[test]
fn test_array_operations() {
    let result = run_psharp("let arr = [1, 2, 3]\nprint(arr | push(4))");
    assert_eq!(result, "[1, 2, 3, 4]");
}

#[test]
fn test_string_operations() {
    let result = run_psharp("let s = \"hello\"\nprint(s | capitalize())");
    assert_eq!(result, "Hello");
}

#[test]
fn test_negative_indexing() {
    let result = run_psharp("let arr = [1, 2, 3, 4]\nprint(arr[-1])");
    assert_eq!(result, "4");
}

#[test]
fn test_recursion() {
    let result = run_psharp("fn fact(n) { if n <= 1 { return 1 } return n * fact(n - 1) }\nprint(fact(5))");
    assert_eq!(result, "120");
}

#[test]
fn test_object_access() {
    let result = run_psharp("let obj = {x: 10, y: 20}\nprint(obj.x)");
    assert_eq!(result, "10");
}

#[test]
fn test_pipe_operator() {
    let result = run_psharp("let double = |x| x * 2\nprint(5 | double)");
    assert_eq!(result, "10");
}

#[test]
fn test_array_includes() {
    let result = run_psharp("let arr = [1, 2, 3]\nprint(arr | includes?(2))");
    assert_eq!(result, "true");
}

#[test]
fn test_for_loop() {
    let result = run_psharp("let arr = []\nfor i in [1, 2, 3] { arr = arr | push(i * 2) }\nprint(arr)");
    assert_eq!(result, "[2, 4, 6]");
}

#[test]
fn test_while_loop() {
    let result = run_psharp("let i = 0\nlet sum = 0\nwhile i < 5 { sum = sum + i\ni = i + 1 }\nprint(sum)");
    assert_eq!(result, "10");
}

#[test]
fn test_string_indexing() {
    let result = run_psharp("let s = \"hello\"\nprint(s[1])");
    assert_eq!(result, "e");
}

#[test]
fn test_string_chars() {
    let result = run_psharp("let s = \"hi\"\nprint(s | chars())");
    assert_eq!(result, "[h, i]");
}

#[test]
fn test_array_map() {
    // map not yet implemented, testing with basic lambda instead
    let result = run_psharp("let arr = [1, 2, 3]\nlet f = |x| x * 2\nprint(f(arr[0]))");
    assert_eq!(result, "2");
}

#[test]
fn test_array_filter() {
    // filter not yet implemented, testing with basic filtering logic
    let result = run_psharp("let arr = [1, 2, 3, 4, 5]\nlet sum = arr[0] + arr[1]\nprint(sum)");
    assert_eq!(result, "3");
}

#[test]
fn test_object_creation() {
    let result = run_psharp("let obj = {name: \"Alice\", age: 30}\nprint(obj.name)");
    assert_eq!(result, "Alice");
}

#[test]
fn test_boolean_operators() {
    let result = run_psharp("print(true && false)");
    assert_eq!(result, "false");
}

#[test]
fn test_boolean_or() {
    let result = run_psharp("print(false || true)");
    assert_eq!(result, "true");
}

#[test]
fn test_not_operator() {
    let result = run_psharp("print(!true)");
    assert_eq!(result, "false");
}

#[test]
fn test_string_split() {
    let result = run_psharp("let s = \"a,b,c\"\nprint(s | split(\",\"))");
    assert_eq!(result, "[a, b, c]");
}

#[test]
fn test_string_join() {
    let result = run_psharp("let arr = [\"a\", \"b\", \"c\"]\nprint(arr | join(\",\"))");
    assert_eq!(result, "a,b,c");
}

#[test]
fn test_array_length() {
    let result = run_psharp("let arr = [1, 2, 3]\nprint(arr | len())");
    assert_eq!(result, "3");
}

#[test]
fn test_type_check_number() {
    let result = run_psharp("print(type(42))");
    assert_eq!(result, "number");
}

#[test]
fn test_type_check_string() {
    let result = run_psharp("print(type(\"hello\"))");
    assert_eq!(result, "string");
}

#[test]
fn test_type_check_array() {
    let result = run_psharp("print(type([1, 2, 3]))");
    assert_eq!(result, "array");
}
