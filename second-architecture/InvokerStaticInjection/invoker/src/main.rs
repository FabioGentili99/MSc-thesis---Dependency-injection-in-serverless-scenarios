use futures::StreamExt;
use once_cell::sync::OnceCell;
use quicli::prelude::*;
use tokio::task::JoinSet;
use std::env;
use std::process::Command;
use std::time::SystemTime;
use log::info;
use log4rs;



#[derive(Debug, Clone)]
struct Service {
    id: String,
    ServiceName: String,
    ServiceAddress: String,
}


fn init_service(service_id: String) -> Service{
    match service_id.as_str() {
        "hello" => {
             Service { id: "hello".to_string(), ServiceName: "hello function".to_string(), ServiceAddress: "http://192.168.17.118:8081/function/hello-fn".to_string() }
        }
        "acl" => {
              Service { id: "acl".to_string(), ServiceName: "access control function".to_string(), ServiceAddress: "http://192.168.17.118:8081/function/acl-fn".to_string() }
        }
        "log" => {
             Service { id: "log".to_string(), ServiceName: "logger".to_string(), ServiceAddress: "http://192.168.17.118:8081/function/log-fn".to_string() }
        }
        default=> {
            Service { id: "default".to_string(), ServiceName: "default function".to_string(), ServiceAddress: "http://192.168.17.118:8081/function/default-fn".to_string() }
        }
    }
}



#[tokio::main]
async fn main() -> CliResult {

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    //Get ENV VAR
    let command: String = env::var("COMMAND").unwrap_or("../go/example-function/acl/handler_acl".to_string());
    let trigger_topic = env::var("TRIGGER").unwrap_or("handler".to_string());
    let output_topic = env::var("OUTPUT").unwrap_or("output".to_string());
    let nats_server = env::var("NATSSERVER").unwrap_or("192.168.17.118:4222".to_string());
    let group = env::var("GROUP").unwrap_or("default".to_string());
    let max_instances: usize = env::var("MAX_INSTANCES").unwrap_or("10".to_string()).parse::<usize>().unwrap();

    
    let service_id = env::var("SERVICEID").unwrap_or("acl".to_string());


    let SERVICE = init_service(service_id.clone());

    println!(
        "starting listening to {}, topic:{}, output:{}, command:{}",
        nats_server, trigger_topic, output_topic, command
    );

    let nc = async_nats::connect(&nats_server).await?;
    let mut sub = nc.queue_subscribe(trigger_topic, group).await.unwrap();
    let mut set = JoinSet::new();




    while let Some(msg) = sub.next().await {
        let command = command.clone();
        let output_topic = output_topic.clone();
        let nc = nc.clone();
        
        
        let sys_time = SystemTime::now();
        while set.len() >= max_instances {
            set.join_next().await; // Wait for a task to finish before spawning a new one
        }

        let fetched_service = SERVICE.clone();
        
        


        set.spawn( async move{
                println!("Received Message: {}", String::from_utf8_lossy(&msg.payload));
            
                //println!("received message on trigger topic");

                //let sys_time = SystemTime::now();
                let output = Command::new("/bin/bash")
                    .arg("-c")
                    .arg(format!("{} '{}'", command, String::from_utf8_lossy(&msg.payload)))        
                    .env("SERVICE_URL", fetched_service.ServiceAddress.clone())            
                    .output()
                    .expect("failed to execute process");
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let lstdout = stdout.trim().to_string();

                println!("output: {:?}", lstdout);

                let new_sys_time = SystemTime::now();

                if output.status.code().unwrap() != 8 {
                    println!("sending message to output");
                    match msg.reply.clone() {
                        Some(topic) => {
                            let _ = nc.publish(topic, lstdout.into());
                        }
                        None => {
                            let _ = nc.publish(output_topic, lstdout.into());
                        }
                    };
                };
                println!("status: {}", output.status);
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

                let difference = new_sys_time
                    .duration_since(sys_time)
                    .expect("Clock may have gone backwards");
                info!("example function executed in {:?}", difference);
                return;
        });
    }

    Ok(())
}