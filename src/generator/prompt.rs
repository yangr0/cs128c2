//! Runs the prompt(in generator context)

// Modules
use crate::take0ver;

// External Libraries
use colored::*;
use prettytable::table;
use rustyline::{error, DefaultEditor};

// Standard Libraries
use std::process::exit;

// Target Info
#[derive(Clone)]
struct Info {
    lhost: String,
    lport: String,
}

pub fn prompt() {
    let mut data = Info {
        lhost: "127.0.0.1".to_string(),
        lport: "1337".to_string(),
    };
    // Console prompts
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("take0ver/generator -> ");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(&line).unwrap();
                // Exits safely
                if line == "exit" {
                    println!("{}", "Exiting...".red().bold());
                    exit(0);

                // Help
                } else if line == "help" {
                    println!("{}", "host=[LISTEN-IP]\nExample: host=1.1.1.1\nport=[LISTEN-PORT]\nExample: port=1337\ninfo - display your current variables\nrun - generates the payload using your current variables\nback - go back to main menu".green().bold())

                // Get Target variable info
                } else if line == "info" {
                    let table = table!(["host", data.lhost], ["port", data.lport]);

                    println!("{}", table.to_string().blue().bold());

                // Sets host variable
                } else if line.to_lowercase().contains("host") {
                    line.retain(|c| !c.is_whitespace());
                    let vec: Vec<&str> = line.split("=").collect();
                    data.lhost = vec[1].to_string();

                // Sets port variable
                } else if line.to_lowercase().contains("port") {
                    line.retain(|c| !c.is_whitespace());
                    let vec: Vec<&str> = line.split("=").collect();
                    data.lport = vec[1].to_string();

                // Calls takev0er::mod.rs to go back home
                } else if line.to_lowercase() == "back" {
                    take0ver::main();

                // Generates payload
                } else if line.to_lowercase() == "run" {
                    super::generate(data.lhost.clone(), data.lport.clone());
                } else {
                    println!(
                        "{}",
                        "Invalid Command. Use \"help\" to list available commands"
                            .red()
                            .bold()
                    )
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
