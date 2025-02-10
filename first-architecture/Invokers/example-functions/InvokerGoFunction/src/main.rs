use quicli::prelude::*;
use tokio::task;
use core::time;
use std::{env, thread};
use std::process::Command;
use std::time::SystemTime;
use log::info;
use log4rs;


#[tokio::main]
async fn main() -> CliResult {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    //Get ENV VAR
    let command: String = env::var("COMMAND").unwrap_or("../go/first-arch-go/functions/log/handler_log".to_string());
    let trigger_topic = env::var("TRIGGER").unwrap_or("handler".to_string());
    let output_topic = env::var("OUTPUT").unwrap_or("output".to_string());
    let nats_server = env::var("NATSSERVER").unwrap_or("192.168.17.118:4222".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());
    let max_instances: usize = env::var("MAX_INSTANCES").unwrap_or("10".to_string()).parse::<usize>().unwrap();


    println!(
        "starting listening to {}, topic:{}, output:{}, command:{}",
        nats_server, trigger_topic, output_topic, command
    );

    let nc = nats::connect(&nats_server)?;
    let sub = nc.queue_subscribe(&trigger_topic, &group)?;
    let mut set = JoinSet::new();
    
    
    for msg in sub.messages() {
        while set.len() >= max_instances {
            set.join_next().await; // Wait for a task to finish before spawning a new one
        }
        let command = command.clone();
        let output_topic = output_topic.clone();
        let nc = nc.clone();
        
        set.spawn( async move{
            println!("Received Message: {}", String::from_utf8_lossy(&msg.data));
            
                println!("received message on trigger topic");

                let sys_time = SystemTime::now();
                let output = Command::new("/bin/bash")
                    .arg("-c")
                    .arg(format!("{} '{}'", command, String::from_utf8_lossy(&msg.data)))                    .output()
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
                info!("example function executed in {:?}", difference);
                thread::sleep(time::Duration::from_millis(5000))
            
        });
    }

    Ok(())
}