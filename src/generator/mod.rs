//! Includes functions to run prompt and call generator

// Modules
mod prompt;
mod generator;

// Calls prompt
pub fn main() {
    prompt::prompt() 
}

pub fn generate(host: String, port: String) {
   generator::generate(host, port)
}
