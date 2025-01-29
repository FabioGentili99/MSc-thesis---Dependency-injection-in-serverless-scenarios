use quicli::prelude::*;
use serde::{Deserialize,Serialize};
use tokio::task;
use core::time;
use std::{env, thread};
use std::process::Command;
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
struct Payload {
    timestamp: String,
    message: String,
    severity: String
}

#[tokio::main]
async fn main() -> CliResult {
    //Get ENV VAR
    let command: String = env::var("COMMAND").unwrap_or("node".to_string());
    let function_path:String = env::var("FUNCTION_PATH").unwrap_or("../function/LOG-js-function/logger.js".to_string());
    let trigger_topic = env::var("TRIGGER").unwrap_or("log".to_string());
    let output_topic = env::var("OUTPUT").unwrap_or("output".to_string());
    let nats_server = env::var("NATSSERVER").unwrap_or("192.168.17.118:4222".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());

    println!(
        "starting listening to {}, topic:{}, output:{}, command:{}",
        nats_server, trigger_topic, output_topic, command
    );

    let nc = nats::connect(&nats_server)?;
    let sub = nc.queue_subscribe(&trigger_topic, &group)?;
    
    
    for msg in sub.messages() {
        let command = command.clone();
        let function_path = function_path.clone();
        let output_topic = output_topic.clone();
        let nc = nc.clone();
        
        task::spawn( async move{
            println!("Received Message: {}", String::from_utf8_lossy(&msg.data));
            if let Ok(payload) =  serde_json::from_str::<Payload>(&String::from_utf8_lossy(&msg.data)) {
                println!("received valid JSON payload: timestamp={:?}, message={:?}, severity={:?}", payload.timestamp, payload.message, payload.severity);
                let sys_time = SystemTime::now();
                let output = Command::new(&command)
                    .arg(function_path)
                    .arg(&payload.timestamp)
                    .arg(&payload.message)
                    .arg(&payload.severity)
                    .output()
                    .expect("failed to execute process");
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let lstdout = stdout.trim();
                println!("output: {:?}", lstdout);
                let new_sys_time = SystemTime::now();

                if output.status.code().unwrap() != 8 {
                    println!("sending message to output");
                    match &msg.reply {
                        Some(topic) => {
                            let _ = nc.publish(&topic, lstdout);
                        }
                        None => {
                            let _ = nc.publish(&output_topic, lstdout);
                        }
                    };
                };
                println!("status: {}", output.status);
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

                let difference = new_sys_time
                    .duration_since(sys_time)
                    .expect("Clock may have gone backwards");
                println!("{:?}", difference);
                thread::sleep(time::Duration::from_millis(5000))
            } else {
                println!("received invalid JSON payload: {:?}", msg.data);
            }
            
        });
    }

    Ok(())
}