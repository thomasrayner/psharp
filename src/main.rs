// Main entry point for P# shell
use anyhow::Result;
use std::env;
use std::fs;

mod lexer;
mod parser;
mod evaluator;
mod types;
mod builtins;
mod repl;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Execute a file
        let filepath = &args[1];
        execute_file(filepath)?;
    } else {
        // Start REPL
        repl::start_repl()?;
    }

    Ok(())
}

fn execute_file(filepath: &str) -> Result<()> {
    let source = fs::read_to_string(filepath)?;
    let tokens = lexer::tokenize(&source)?;
    let ast = parser::parse(tokens)?;
    let mut evaluator = evaluator::Evaluator::new();
    evaluator.eval_program(&ast)?;
    Ok(())
}
