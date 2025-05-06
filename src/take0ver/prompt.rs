//! Runs prompt(in home context)

// External libraries
use colored::*;
use rustyline::{error, DefaultEditor};

// Standard Libraries
use std::process::exit;

// Console prompts
pub fn prompt() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("take0ver -> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line).unwrap();
                super::intline(line);
            }
            Err(error::ReadlineError::Interrupted) => {
                println!("{}", "CTRL-C".red().bold());
                exit(0)
            }
            Err(error::ReadlineError::Eof) => {
                println!("{}", "CTRL-D".red().bold());
                exit(0)
            }
            Err(err) => {
                println!("Error: {:?}", err);
                exit(0)
            }
        }
    }
}
