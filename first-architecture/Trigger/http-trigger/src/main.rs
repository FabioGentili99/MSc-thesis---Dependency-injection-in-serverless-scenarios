use actix_web::{post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, io::Read};

#[derive(Deserialize, Serialize)]
struct PublishRequest {
    message: String,
}

async fn send(
    function: String,
    map: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
) -> Result<(), std::io::Error> {
    let nats_url = env::var("NATS_URL").unwrap_or("192.168.17.118:4222".to_string());
    let nc = nats::connect(nats_url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    nc.publish(&map.get(&function).unwrap().to_string(), req.message.clone()).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    nc.close();
    Ok(())
}


async fn query(
    function: String,
    map: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
) -> Result<nats::Message, std::io::Error> {
    let nats_url = env::var("NATS_URL").unwrap_or("192.168.17.118:4222".to_string());
    let nc = nats::connect(nats_url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let response = nc.request(&map.get(&function).unwrap().to_string(), req.message.clone()).unwrap();
    nc.close();
    Ok(response)
}


#[post("/asyncfunction/{fun}")]
async fn async_invoke(
    routes: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    fun: web::Path<String>,
) -> impl Responder {
    let _ = send(fun.into_inner(), routes, req).await;
    HttpResponse::Ok().body("ok")
}

#[post("/function/{fun}")]
async fn sync_invoke(
    routes: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    fun: web::Path<String>,
) -> impl Responder {
    let message = query(fun.into_inner(), routes, req).await.unwrap();
    HttpResponse::Ok().body(message.data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes: Data<DashMap<String, String>> = Data::new(DashMap::new());
    let mut file = File::open("config.json")?; // Open the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read file into a string

    let data: HashMap<String, String> = serde_json::from_str(&contents)?; // Parse JSON into HashMap
    println!("{:?}", data);
    for (key, value) in data {
        routes.insert(key, value);
    };

    HttpServer::new(move || {
        App::new()
            .app_data(routes.clone())
            .service(sync_invoke)
            .service(async_invoke)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
