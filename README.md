# P# - The Joy of Shell Scripting

P# is a modern, elegant shell language that fixes everything wrong with PowerShell while maintaining the productivity benefits of a powerful scripting environment.

## Why P#?

**PowerShell Problems P# Fixes:**
- ✨ Clean, intuitive syntax inspired by Python and modern languages
- 🚀 Blazing fast execution (written in Rust)
- 💎 Strong typing with excellent type inference
- 🔧 First-class piping with proper object handling
- 😍 No more `$` sigils or `{}` syntax nightmares
- 📦 Rich built-in data structures (arrays, objects, strings)
- 🎯 Predictable behavior - what you see is what you get
- 🔐 Memory-safe implementation

## Installation

```bash
cargo build --release
./target/release/psharp
```

## Quick Start

### Interactive REPL

```bash
$ p#
Welcome to P# Shell v0.1.0
p# > print("Hello, World!")
Hello, World!

p# > let x = 42
p# > print(x * 2)
84

p# > [1, 2, 3] | len()
3
```

### Running P# Scripts

```bash
p# script.ps
```

## Language Features

### Variables

Clean, simple variable declaration:

```p#
let name = "Alice"
let count = 42
let items = [1, 2, 3, 4, 5]
let person = { name: "Bob", age: 30, email: "bob@example.com" }
```

### Data Types

P# has a rich type system:

- **null** - Nothing/empty value
- **bool** - true or false
- **number** - All numeric values (42, 3.14)
- **string** - Text ("hello", 'world')
- **array** - Ordered collections [1, 2, "three"]
- **object** - Key-value maps { name: "Alice", age: 25 }
- **function** - First-class functions

### Functions

Define reusable functions:

```p#
fn greet(name) {
    print("Hello, " + name)
}

greet("World")

# Functions with multiple statements
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print(factorial(5))  # 120
```

### Control Flow

#### If Statements

```p#
let age = 18
if age >= 18 {
    print("You are an adult")
} else {
    print("You are a minor")
}
```

#### While Loops

```p#
let i = 0
while i < 5 {
    print(i)
    i = i + 1
}
```

#### For Loops

```p#
# Loop over ranges
for item in [1, 2, 3, 4, 5] {
    print(item)
}

# Loop over strings
for char in "hello" {
    print(char)
}
```

### The Pipe Operator

The pipe operator `|` is central to P# - pass values through transformations:

```p#
let nums = [1, 2, 3, 4, 5]

# Chain operations
nums | len() | print()  # 5

# Create ranges
range(1, 10) | print()

# Process strings
"hello world" | upper() | print()  # HELLO WORLD
```

### Operators

**Arithmetic:**
```p#
1 + 2          # Addition
5 - 3          # Subtraction
4 * 5          # Multiplication
10 / 2         # Division
10 % 3         # Modulo
2 ** 8         # Power
```

**Comparison:**
```p#
1 < 2          # Less than
2 <= 2         # Less than or equal
3 > 1          # Greater than
3 >= 3         # Greater than or equal
5 == 5         # Equality
5 != 3         # Not equal
```

**Logical:**
```p#
true and false # Logical AND
true or false  # Logical OR
not true       # Logical NOT
```

## Built-in Functions

### String Functions

```p#
len("hello")              # 5
upper("hello")            # "HELLO"
lower("HELLO")            # "hello"
trim("  hello  ")         # "hello"
split("a,b,c", ",")       # ["a", "b", "c"]
join(["a", "b", "c"], ",") # "a,b,c"
contains("hello", "ell")  # true
starts_with("hello", "he") # true
ends_with("hello", "lo")  # true
replace("hello", "l", "L") # "heLLo"
```

### Array Functions

```p#
len([1, 2, 3])           # 3
reverse([1, 2, 3])       # [3, 2, 1]
sort([3, 1, 2])          # [1, 2, 3]
contains([1, 2, 3], 2)   # true
join([1, 2, 3], ",")     # "1,2,3"
```

### Math Functions

```p#
abs(-5)                  # 5
sqrt(16)                 # 4
floor(3.7)               # 3
ceil(3.2)                # 4
round(3.14159)           # 3
pow(2, 3)                # 8
min([1, 5, 3])           # 1
max([1, 5, 3])           # 5
sum([1, 2, 3])           # 6
```

### Object Functions

```p#
let obj = { name: "Alice", age: 30 }
keys(obj)                # ["name", "age"]
values(obj)              # ["Alice", 30]
```

### I/O Functions

```p#
print("Hello")           # Print to stdout
print(123)
read_file("file.txt")    # Read entire file
write_file("file.txt", "content") # Write to file
```

### System Functions

```p#
exec("ls -la")           # Execute shell command
exists("file.txt")       # Check if file exists
ls()                     # List current directory
ls("/path/to/dir")       # List directory
```

### Type Functions

```p#
type(42)                 # "number"
type("hello")            # "string"
type([1, 2, 3])          # "array"
type({ a: 1 })           # "object"
to_string(42)            # "42"
to_number("42")          # 42
```

## Examples

### Example 1: Simple Script

```p#
# Count files in directory
let files = ls(".")
print("Found ")
print(len(files))
print(" files")
```

### Example 2: Data Processing

```p#
let data = [1, 2, 3, 4, 5]
let sum = sum(data)
let avg = sum / len(data)
print("Average: ")
print(avg)
```

### Example 3: System Administration

```p#
fn list_files_in_dir(dir) {
    let files = ls(dir)
    for f in files {
        print(f)
    }
}

list_files_in_dir(".")
```

### Example 4: Text Processing

```p#
let text = read_file("data.txt")
let lines = split(text, "\n")
let count = len(lines)
print("File has ")
print(count)
print(" lines")
```

### Example 5: Control Flow

```p#
fn is_prime(n) {
    if n < 2 { return false }
    
    let i = 2
    while i * i <= n {
        if n % i == 0 { return false }
        i = i + 1
    }
    
    return true
}

for num in range(1, 20) {
    if is_prime(num) {
        print(num)
    }
}
```

## Advantages Over PowerShell

| Feature | PowerShell | P# |
|---------|-----------|-----|
| Syntax | Complex, inconsistent | Clean, Python-like |
| Learning Curve | Steep | Gentle |
| Performance | Moderate | Blazing Fast |
| Type System | Weak | Strong with inference |
| Piping | Object-based | Value-based |
| Memory Safety | Not guaranteed | Guaranteed (Rust) |
| Scripting Speed | Slower | Faster |
| Code Readability | Poor | Excellent |

## Philosophy

P# is built on these principles:

1. **Simplicity** - Easy to learn and understand
2. **Consistency** - Predictable behavior
3. **Performance** - Compiled Rust implementation
4. **Safety** - Memory-safe operations
5. **Interoperability** - Easy system integration
6. **Joy** - Scripting should be fun!

## Architecture

P# consists of:

- **Lexer** (`lexer.rs`) - Tokenizes source code
- **Parser** (`parser.rs`) - Builds abstract syntax tree
- **Type System** (`types.rs`) - Runtime value representation
- **Evaluator** (`evaluator.rs`) - Executes AST with environments
- **Built-ins** (`builtins.rs`) - Standard library functions
- **REPL** (`repl.rs`) - Interactive shell experience

## Future Enhancements

- [ ] Module system with imports
- [ ] Pattern matching
- [ ] List comprehensions
- [ ] Async/await support
- [ ] Type annotations
- [ ] Better error messages with line numbers
- [ ] Standard library expansion
- [ ] Package manager
- [ ] Debugger
- [ ] LSP integration for IDE support

## Contributing

P# is open source and welcomes contributions!

## License

MIT

---

**Welcome to the joy of shell scripting with P#** 🎉
