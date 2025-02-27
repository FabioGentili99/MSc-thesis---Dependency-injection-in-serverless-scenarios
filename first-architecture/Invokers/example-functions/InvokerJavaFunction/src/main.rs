use quicli::prelude::*;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::task::JoinSet;
use std::env;
use std::time::SystemTime;
use log::info;
use log4rs;


#[tokio::main]
async fn main() -> CliResult {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    //Get ENV VAR
    let command: String = env::var("COMMAND").unwrap_or("java -jar ../java/first-arch-java/build/libs/handler_log-1.0-SNAPSHOT.jar".to_string());
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
        let command = command.clone();
        let output_topic = output_topic.clone();
        let nc = nc.clone();
        
        let sys_time = SystemTime::now();
        while set.len() >= max_instances {
            set.join_next().await; // Wait for a task to finish before spawning a new one
        }
        
        set.spawn( async move{
            println!("Received Message: {}", String::from_utf8_lossy(&msg.data));
            
            //println!("received message on trigger topic");

            //let sys_time = SystemTime::now();
            let mut child = Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("{} '{}'", command, String::from_utf8_lossy(&msg.data)))      
                .stdout(std::process::Stdio::piped())  // Capture stdout              
                .spawn()
                .expect("failed to execute process");

            // Ensure the child has stdout available
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let mut reader = BufReader::new(stdout).lines();

            let mut output = String::new();

            // Read output line by line
            while let Some(line) = reader.next_line().await.unwrap() {
                output.push_str(&line);
                output.push('\n');  // Preserve line breaks
            }

            // Ensure the child process exits properly
            let status = child.wait().await;


            let output = output.trim();

            println!("output: {:?}", output);

            let new_sys_time = SystemTime::now();

            if status.unwrap().code().unwrap() != 8 {
                println!("sending message to output");
                match &msg.reply {
                    Some(topic) => {
                        let _ = nc.publish(&topic, output);
                    }
                    None => {
                        let _ = nc.publish(&output_topic, output);
                    }
                };
            };

            let difference = new_sys_time
                .duration_since(sys_time)
                .expect("Clock may have gone backwards");
            info!("example function executed in {:?}", difference);
            return;
        });
    }

    Ok(())
}