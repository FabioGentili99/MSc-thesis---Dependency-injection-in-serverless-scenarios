mod injector;

use futures::StreamExt;
use injector::{get_mongo_client, ServiceRegistry};
use quicli::prelude::*;
use tokio::task::JoinSet;
use std::env;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::info;
use log4rs;


#[tokio::main]
async fn main() -> CliResult {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    //Get ENV VAR
    let command: String = env::var("COMMAND").unwrap_or("../go/example-function/acl/handler_acl".to_string());
    let trigger_topic = env::var("TRIGGER").unwrap_or("handler".to_string());
    let output_topic = env::var("OUTPUT").unwrap_or("output".to_string());
    let nats_server = env::var("NATSSERVER").unwrap_or("192.168.17.118:4222".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());
    let max_instances: usize = env::var("MAX_INSTANCES").unwrap_or("5".to_string()).parse::<usize>().unwrap();

    let mongo_url = env::var("MONGO").unwrap_or("mongodb://192.168.17.118:27017".to_string());
    //let db_name = env::var("DBNAME").unwrap_or("services".to_string());
    //let collection_name = env::var("COLLECTIONNAME").unwrap_or("services".to_string());
    let service_id = env::var("SERVICEID").unwrap_or("acl".to_string());


    println!(
        "starting listening to {}, topic:{}, output:{}, command:{}",
        nats_server, trigger_topic, output_topic, command
    );

    let nc = async_nats::connect(&nats_server).await?;
    let mut sub = nc.queue_subscribe(trigger_topic, group).await.unwrap();
    let mut set = JoinSet::new();

    let client = get_mongo_client(&mongo_url).await;
    let repo = ServiceRegistry::new(client);
    
    while let Some(msg) = sub.next().await {
        let command = command.clone();
        let output_topic = output_topic.clone();
        let nc = nc.clone();
        
        
        let sys_time = UNIX_EPOCH + Duration::from_millis(String::from_utf8_lossy(&msg.payload).parse::<u64>().expect("Failed to parse message data as u64"));
        while set.len() >= max_instances {
            set.join_next().await; // Wait for a task to finish before spawning a new one
        }


        //dynamic dependency injection
        let retrieval_start = SystemTime::now();
        let fetched_service = repo.get_service(&service_id).await.unwrap();
        let retrieval_end = SystemTime::now();
        let retrieval_duration = retrieval_end
            .duration_since(retrieval_start)
            .expect("Clock may have gone backwards");
        info!("Read from MongoDB executed in {:?}", retrieval_duration);
        


        set.spawn( async move{
            println!("Received Message: {}", String::from_utf8_lossy(&msg.payload));
            
            //println!("received message on trigger topic");

            //let sys_time = SystemTime::now();
            let mut child = Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("{} '{}'", command, String::from_utf8_lossy(&msg.payload)))      
                .env("SERVICE", fetched_service.ServiceAddress)
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


            let output = output.trim().to_string();

            println!("output: {:?}", output);

            let new_sys_time = SystemTime::now();

            if status.unwrap().code().unwrap() != 8 {
                println!("sending message to output");
                match &msg.reply {
                    Some(topic) => {
                        let _ = nc.publish(topic.clone(), output.into());
                    }
                    None => {
                        let _ = nc.publish(output_topic, output.into());
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