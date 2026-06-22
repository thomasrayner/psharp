# P# - The Joy of Shell Scripting

P# is a modern, elegant shell language that fixes everything wrong with PowerShell while maintaining the productivity benefits of a powerful scripting environment.

**Current Version:** 0.2.0 - Quality & Security Release

## 🎉 What's New in v0.2

### Critical Fixes
- ✅ **Recursive Functions** - Functions can now call themselves (factorial, fibonacci, etc.)
- ✅ **Security Hardened** - Removed shell injection vulnerability, restricted file access to current directory
- ✅ **Resource Protection** - Recursion depth limit (1000) prevents stack overflow
- ✅ **Negative Indexing** - Python-style `arr[-1]` for last element

### 30+ New Builtin Functions
- **Array:** push, pop, shift, unshift, sort, unique, flatten, compact, slice, find, index_of
- **String:** chars, lines, capitalize, indent, pad_left, pad_right
- **Predicates:** includes?, empty?, any?, all?
- **Aggregation:** avg (and sum, min, max improvements)

### Examples
- `examples/recursion.ps` - Working recursive algorithms
- `examples/array_operations.ps` - All 20+ array functions
- `examples/string_operations.ps` - All 15+ string functions
- `examples/advanced_features.ps` - Complete feature showcase

**See [IMPROVEMENTS.md](IMPROVEMENTS.md) for complete details.**

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

## 🌍 P# Ecosystem

P# is a complete ecosystem with multiple components working together:

### Core Language
**Repository:** [thomasrayner/psharp](https://github.com/thomasrayner/psharp)  
**Description:** The P# language implementation in Rust. Includes lexer, parser, evaluator, REPL, and 40+ built-in functions.  
**Features:** Variables, functions, control flow, data structures, system integration  
**Status:** ✅ Production ready (v0.1.0)

### Language Server Protocol (LSP)
**Repository:** [thomasrayner/psharp-lsp](https://github.com/thomasrayner/psharp-lsp)  
**Description:** Standalone LSP server for P# language support in any editor.  
**Features:** Auto-completion, hover documentation, symbol navigation, error diagnostics  
**Status:** ✅ Fully functional

### VSCode Extension
**Repository:** [thomasrayner/psharp-vscode](https://github.com/thomasrayner/psharp-vscode)  
**Description:** Official VSCode extension with syntax highlighting and LSP integration.  
**Features:** Syntax coloring, IntelliSense, command palette integration  
**Installation:** Search "P#" in VSCode marketplace  
**Status:** ✅ Available on marketplace

### Neovim Plugin
**Repository:** [thomasrayner/psharp-nvim](https://github.com/thomasrayner/psharp-nvim)  
**Description:** Neovim plugin for P# with syntax highlighting and LSP setup.  
**Features:** Native Vim syntax, LSP configuration, ftplugin setup  
**Installation:** Add to your Neovim plugin manager  
**Status:** ✅ Production ready

### Web & Marketing
**Repository:** [thomasrayner/psharp-web](https://github.com/thomasrayner/psharp-web)  
**Description:** Complete marketing ecosystem including interactive playground, tutorials, and community materials.  
**Features:**
  - 🎮 **Interactive Playground** - Try P# in browser without installation
  - 📚 **50+ Examples** - Copy-paste scripts for every use case
  - 📖 **Quick Cheat Sheet** - Language reference guide
  - 🎬 **Video Tutorials** - 6 production-ready scripts (3-5 min each)
  - 🎨 **Branding** - Complete design system with Harmony mascot
  - 🏆 **Achievement System** - 25 badges across 5 levels
  - 💬 **Community Templates** - GitHub Discussions setup
  - 🎁 **Assets** - SVG badges and graphics  
**Status:** ✅ Launch ready

---

## 🚀 Getting Started

### 1. Install P#
```bash
git clone https://github.com/thomasrayner/psharp
cd psharp
cargo build --release
./target/release/psharp
```

### 2. Try in Browser (No Installation)
Visit the **[Interactive Playground](https://thomasrayner.github.io/psharp-web/playground.html)** and code P# instantly!

### 3. Install IDE Support

**VSCode:**
- Search for "P#" in marketplace
- Click Install
- Reload VSCode

**Neovim:**
- Install `psharp-nvim` plugin via your plugin manager
- Run `:PlugInstall` (or equivalent)

### 4. Learn with Examples
Browse the **[50+ Examples](https://github.com/thomasrayner/psharp-web/blob/main/EXAMPLES.md)** or use the **[Quick Cheat Sheet](https://github.com/thomasrayner/psharp-web/blob/main/CHEATSHEET.md)**

### 5. Join the Community
- 💬 **[GitHub Discussions](https://github.com/thomasrayner/psharp/discussions)** - Ask questions, share projects
- 🎓 **[Video Tutorials](https://github.com/thomasrayner/psharp-web/blob/main/VIDEO_TUTORIALS.md)** - Learn step-by-step
- 🏆 **[Achievements](https://github.com/thomasrayner/psharp-web/blob/main/ACHIEVEMENTS.md)** - Earn badges as you learn

---

## 📚 Documentation

| Resource | Purpose |
|----------|---------|
| [Interactive Playground](https://thomasrayner.github.io/psharp-web/playground.html) | Try P# in browser |
| [50+ Examples](https://github.com/thomasrayner/psharp-web/blob/main/EXAMPLES.md) | Learn by doing |
| [Cheat Sheet](https://github.com/thomasrayner/psharp-web/blob/main/CHEATSHEET.md) | Quick reference |
| [Video Tutorials](https://github.com/thomasrayner/psharp-web/blob/main/VIDEO_TUTORIALS.md) | Step-by-step guides |
| [Branding Guide](https://github.com/thomasrayner/psharp-web/blob/main/BRANDING.md) | Design system |
| [Achievement System](https://github.com/thomasrayner/psharp-web/blob/main/ACHIEVEMENTS.md) | Gamification |
| [Community Setup](https://github.com/thomasrayner/psharp-web/blob/main/GITHUB_DISCUSSIONS.md) | Discussion templates |

---

## 🎵 Philosophy

P# is built on the principle that **scripting should be joyful**.

**Our Philosophy:**
1. **Simplicity** - Easy to learn and understand
2. **Consistency** - Predictable behavior
3. **Performance** - Compiled Rust implementation
4. **Safety** - Memory-safe operations
5. **Interoperability** - Easy system integration
6. **Joy** - Scripting should be fun! 🎵

Every tool, tutorial, and interaction celebrates the joy of coding.

---

## 🎨 Meet Harmony

**Harmony** is the friendly P# mascot - a joyful musical note character representing the delight and music of scripting. You'll see Harmony throughout the community, tutorials, and documentation celebrating your progress!

---

## Contributing

P# is open source and welcomes contributions!

**Ways to Contribute:**
- 🐛 Report bugs and suggest features
- 💻 Submit code improvements
- 📚 Write examples and documentation
- 🎬 Create video tutorials
- 🤝 Help in GitHub Discussions
- 🎨 Design Harmony expressions and graphics

See individual repository CONTRIBUTING.md files for details.

---

## Repository Map

```
P# Ecosystem
├── psharp                    (Core language)
├── psharp-lsp              (LSP server)
├── psharp-vscode           (VSCode extension)
├── psharp-nvim             (Neovim plugin)
└── psharp-web              (Docs, playground, community)
    ├── playground.html
    ├── EXAMPLES.md
    ├── CHEATSHEET.md
    ├── VIDEO_TUTORIALS.md
    ├── BRANDING.md
    ├── ACHIEVEMENTS.md
    ├── STICKERS.svg
    └── GITHUB_DISCUSSIONS.md
```

---

## License

MIT - All repositories use MIT license.

---

**Welcome to the joy of shell scripting with P#!** 🎵

**Get started:** 
- 🎮 [Try Playground](https://thomasrayner.github.io/psharp-web/playground.html)
- 📖 [Read Docs](https://github.com/thomasrayner/psharp-web/blob/main/README_MARKETING.md)
- 💬 [Join Community](https://github.com/thomasrayner/psharp/discussions)
