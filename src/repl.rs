use crate::lexer::tokenize;
use crate::parser::parse;
use crate::evaluator::Evaluator;
use anyhow::Result;
use rustyline::DefaultEditor;

pub fn start_repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut evaluator = Evaluator::new();

    println!("Welcome to P# Shell v0.1.0");
    println!("Type 'exit' to quit, 'help' for commands\n");

    loop {
        let readline = rl.readline("p# > ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                
                if trimmed.is_empty() {
                    continue;
                }
                
                if trimmed == "exit" || trimmed == "quit" {
                    println!("Goodbye!");
                    break;
                }

                if trimmed == "help" {
                    print_help();
                    continue;
                }

                rl.add_history_entry(trimmed)?;

                match execute_repl_line(trimmed, &mut evaluator) {
                    Ok(output) => {
                        if !output.is_empty() {
                            println!("{}", output);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("\nGoodbye!");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn execute_repl_line(line: &str, evaluator: &mut Evaluator) -> Result<String> {
    let tokens = tokenize(line)?;
    let statements = parse(tokens)?;
    
    for stmt in statements {
        evaluator.eval_statement(&stmt)?;
    }

    Ok(String::new())
}

fn print_help() {
    println!("P# Shell Commands:");
    println!("  exit, quit      - Exit the shell");
    println!("  help            - Show this help message");
    println!("\nBuilt-in Functions:");
    println!("  print(...)      - Print to stdout");
    println!("  len(val)        - Get length of string/array/object");
    println!("  type(val)       - Get type of value");
    println!("  range(n)        - Create array 0..n");
    println!("  keys(obj)       - Get object keys");
    println!("  values(obj)     - Get object values");
    println!("\nExamples:");
    println!("  p# > let x = 5");
    println!("  p# > print(x)");
    println!("  p# > [1, 2, 3] | len()");
}
