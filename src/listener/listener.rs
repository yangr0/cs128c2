//! Starts a TCP listener

// External Libraries
use colored::*;

// Standard Libraries
use std::net::TcpListener;
use std::process::exit;

// Everything below is the listener
fn thread<R, W>(mut r: R, mut w: W) -> std::thread::JoinHandle<()>
where
    R: std::io::Read + Send + 'static,
    W: std::io::Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let len = match r.read(&mut buffer) {
                Ok(read) => read,
                Err(_) => exit(1),
            };
            if len == 0 {
                println!("{}", "shell closed\n".red().bold());
                exit(1);
            }
            match w.write(&buffer[..len]) {
                Ok(write) => write,
                Err(_) => exit(1),
            };
            match w.flush() {
                Ok(output) => output,
                Err(e) => println!("{}", e),
            };
        }
    })
}

pub fn listen(host: String, port: String) {
    let port = match port.parse::<u16>() {
        Ok(a) => a,
        Err(_) => panic!(),
    };

    let target = match TcpListener::bind((host.clone(), port)) {
        Ok(bind) => bind,
        Err(_) => panic!("failed to bind to {}:{}", host, port),
    };
    let stream = match target.accept() {
        Ok((accept, addr)) => {
            println!(
                "{}{}{}",
                "Connection recieved from ".green().bold(),
                addr.to_string().green().bold(),
                "! In a shell!".green().bold()
            );
            accept
        }
        Err(_) => panic!(),
    };
    let stdin = thread(std::io::stdin(), stream.try_clone().unwrap());
    let stdout = thread(stream, std::io::stdout());
    match stdin.join() {
        Ok(stdin) => stdin,
        Err(_) => println!("stdin failed"),
    };
    match stdout.join() {
        Ok(stdout) => stdout,
        Err(_) => println!("stdout failed"),
    };
}
