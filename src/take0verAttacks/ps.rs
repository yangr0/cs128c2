use std::process::Command;

fn main() {
    let cmd = "/c echo \"testing\"";
    let command = Command::new("powershell.exe")
        .args(&["/c", cmd])
        .output()
        .expect("failed to run command");

    let output = match String::from_utf8(command.stdout) {
        Ok(output) => output,
        Err(_) => "Failed to run command".to_owned(),
    };
    println!("{}", output)
}
