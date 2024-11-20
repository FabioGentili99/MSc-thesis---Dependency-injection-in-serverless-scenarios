use quicli::prelude::*;
use std::env;
use std::process::Command;
use std::time::SystemTime;

fn main() -> CliResult {
    //Get ENV VAR
    let command = env::var("COMMAND").unwrap_or("echo ciao".to_string());
    let trigger_topic = env::var("TRIGGER").unwrap_or("trigger".to_string());
    let output_topic = env::var("OUTPUT").unwrap_or("output".to_string());
    let nats_server = env::var("NATSSERVER").unwrap_or("nats".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());

    let cm = "node ./examples/main.js";
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg(&cm)
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let status = output.status;

    println!("output:{}, status:{}", stdout, status);

    Ok(())
}
