//! Starts prompt(in listener context)

// Modules
use crate::take0ver;

// External Libraries
use rustyline::{Editor, error};
use prettytable::{table, row, cell};
use colored::*;

// Standard Libraries
use std::process::exit;

// Target info
#[derive(Clone)]
struct Info {
    lhost: String,
    lport: String,
}

// Main
pub fn prompt() {
    let mut data = Info {
        lhost: "0.0.0.0".to_string(),
        lport: "1337".to_string(),
    };
    // Console prompts
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("take0ver/listener -> ");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(line.clone());
                // Exits safely
                if line == "exit" {
                    println!("{}", "Exiting...".red().bold());
                    exit(0);

                // Help
                } else if line == "help" {
                    println!("{}", "host=[LISTEN-IP]\nExample: host=0.0.0.0\nport=[LISTEN-PORT]\nExample: port=1337\ninfo - display your current variables\nrun - starts the listener using current variables".green().bold())

                // Get target info
                } else if line == "info" {
                    let table = table!(["host", data.lhost],
                        ["port", data.lport]);
                        
                    println!("{}", table.to_string().blue().bold())

                // Sets listener host based on user input
                } else if line.to_lowercase().contains("host") {
                    line.retain(|c| !c.is_whitespace());
                    let vec: Vec<&str> = line.split("=").collect();
                    data.lhost = vec[1].to_string();

                // Sets listener port based on user input
                } else if line.to_lowercase().contains("port") {
                    line.retain(|c| !c.is_whitespace());
                    let vec: Vec<&str> = line.split("=").collect();
                    data.lport = vec[1].to_string();

                // Calls take0ver::mod.rs to back to home
                } else if line.to_lowercase() == "back" {
                    take0ver::main();

                // Starts the listener
                } else if line == "run" {
                    println!("{}{}:{}...", "Started listener on ".green().bold(), data.lhost.blue().bold(), data.lport.blue().bold());
                    println!("{}", "Do [CTRL-C] to cancel".yellow().bold());
                    super::listen(data.lhost.clone(), data.lport.clone());

                } else { println!("{}", "Invalid Command. Use \"help\" to list available commands".red().bold());
                }
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
