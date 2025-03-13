use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Semaphore;
use tokio::task;

#[tokio::main]
async fn main() {
    let url = "http://192.168.17.118:8081/asyncfunction/example-fn".to_string();
    let headers = vec![("Content-Type".to_string(), "application/json".to_string())];
    let mode = "incremental"; // Choose "constant" or "incremental"
    let duration = 10; // Duration in seconds (only for constant mode)
    let start_rate = 1; // Requests per second (incremental start rate)
    let peak_rate = 50; // Requests per second (incremental peak rate)
    let increment_per_second = 2; // Incremental increase per second
    let client = Arc::new(Client::new());
    
    if mode == "constant" {
        run_constant_rate(client, &url, headers.clone(), peak_rate, duration).await;
    } else if mode == "incremental" {
        run_incremental_rate(client, &url,  headers.clone(), start_rate, peak_rate, increment_per_second).await;
    }
}

async fn send_request(client: Arc<Client>, url: &str, headers: Vec<(String, String)>) {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis().to_string();
    let data = serde_json::json!({ "message": millis});
    let mut request = client.post(url).json(&data);
    for (key, value) in headers {
        request = request.header(key, value);
    }
    match request.send().await {
        Ok(response) => println!("Response: {}", response.status()),
        Err(e) => eprintln!("Request failed: {}", e),
    }
}

async fn run_constant_rate(
    client: Arc<Client>,
    url: &str,
    headers: Vec<(String, String)>,
    rate: usize,
    duration: usize,
) {
    let semaphore = Arc::new(Semaphore::new(rate));
    for _ in 0..duration {
        let sem = semaphore.clone();
        for _ in 0..rate {
            let permit = sem.clone().acquire_owned().await.unwrap();
            let client = client.clone();
            let url = url.to_string();
            let headers = headers.clone();
            task::spawn(async move {
                send_request(client, &url, headers).await;
                drop(permit);
            });
        }
        thread::sleep(Duration::from_secs(1));
    }
}

async fn run_incremental_rate(
    client: Arc<Client>,
    url: &str,
    headers: Vec<(String, String)>,
    start_rate: usize,
    peak_rate: usize,
    increment_per_second: usize,
) {
    let mut rate = start_rate;
    while rate < peak_rate {
        run_constant_rate(client.clone(), url, headers.clone(), rate, 1).await;
        rate += increment_per_second;
        if rate > peak_rate {
            rate = peak_rate;
        }
    }
}
