//! Runs prompt and interprets user input

// Modules
mod prompt;
use crate::generator;
use crate::listener;

// External libraries
use colored::*;

// Standard libraries
use std::process::exit;

// Calls prompt
pub fn main() {
    prompt::prompt();
}

// Interpret user input
fn intline(line: String) {
    // Help 
    if line == "help" {
        println!("{}", "generator - Payload Generator\nlistener - Reverse Shell Listener".green().bold())

    // Exit
    } else if line.to_lowercase() == "exit" {
        println!("{}", "Exiting...".red().bold());
        exit(0)

    // Goes into listener context
    } else if line.to_lowercase() == "listener" {
        listener::main()

    // Goes into generator context
    } else if line.to_lowercase() == "generator" {
        generator::main()

    } else {
        println!("{}", "Invalid Command. Use \"help\" to list available commands".red().bold())
    }
}
