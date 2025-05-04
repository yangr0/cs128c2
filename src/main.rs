// Prints banner and starts the console

// Modules
mod generator;
mod listener;
mod take0ver;

// External libraries
use colored::*;

// Banner
const BANNER: &str = r#"
▄▄▄█████▓ ▄▄▄       ██ ▄█▀▓█████ ██▒   █▓▓█████  ██▀███  
▓  ██▒ ▓▒▒████▄     ██▄█▒ ▓█   ▀▓██░   █▒▓█   ▀ ▓██ ▒ ██▒
▒ ▓██░ ▒░▒██  ▀█▄  ▓███▄░ ▒███   ▓██  █▒░▒███   ▓██ ░▄█ ▒
░ ▓██▓ ░ ░██▄▄▄▄██ ▓██ █▄ ▒▓█  ▄  ▒██ █░░▒▓█  ▄ ▒██▀▀█▄  
  ▒██▒ ░  ▓█   ▓██▒▒██▒ █▄░▒████▒  ▒▀█░  ░▒████▒░██▓ ▒██▒
  ▒ ░░    ▒▒   ▓▒█░▒ ▒▒ ▓▒░░ ▒░ ░  ░ ▐░  ░░ ▒░ ░░ ▒▓ ░▒▓░
    ░      ▒   ▒▒ ░░ ░▒ ▒░ ░ ░  ░  ░ ░░   ░ ░  ░  ░▒ ░ ▒░
  ░        ░   ▒   ░ ░░ ░    ░       ░░     ░     ░░   ░ 
               ░  ░░  ░      ░  ░     ░     ░  ░   ░     
                                     ░                   

            quin | thomas | raymond
"#;

fn main() {
    println!("{}", BANNER.magenta().bold());
    take0ver::main();
}
