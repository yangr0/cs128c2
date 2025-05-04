//! Includes functions to call the prompt and call the listener function

// Modules
mod listener;
mod prompt;

// Main
pub fn main() {
    prompt::prompt()
}

pub fn listen(host: String, port: String) {
    listener::listen(host, port)
}
