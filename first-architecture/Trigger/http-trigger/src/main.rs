mod nats_pool;

use actix_web::{middleware, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use bb8::PooledConnection;
use dashmap::DashMap;
use log::info;
use nats_pool::{create_nats_pool, NatsPool};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, io::Read, sync::Arc, time::  SystemTime};

#[derive(Deserialize, Serialize)]
struct PublishRequest {
    message: String,
}

async fn send(
    function: String,
    map: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    pool: web::Data<Arc<NatsPool>>
) -> Result<(), std::io::Error> {
    let pool = pool.get_ref();
    let nc: PooledConnection<'_, _> = pool.get().await.expect("Failed to get NATS connection");

    nc.publish(map.get(&function).unwrap().to_string(), req.message.clone().into()).await.unwrap();
    Ok(())
}


async fn query(
    function: String,
    map: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    pool: web::Data<Arc<NatsPool>>
) -> Result<async_nats::Message, async_nats::Error> {
    let pool = pool.get_ref();
    let nc: PooledConnection<'_, _> = pool.get().await.expect("Failed to get NATS connection");
    /* 
    let mut request = async_nats::Request::new();
    request = request.timeout(Some(Duration::from_secs(10)));
    request = request.payload(req.message.clone().into());
    */
    let response = nc.request(map.get(&function).unwrap().to_string(), req.message.clone().into()).await.unwrap();
    Ok(response)
}


#[post("/asyncfunction/{fun}")]
async fn async_invoke(
    routes: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    fun: web::Path<String>,
    pool: web::Data<Arc<NatsPool>>
) -> impl Responder {
    let function_name = fun.into_inner();
    let _ = send(function_name.clone(), routes, req, pool).await;
    info!("handling request to async function: {}", function_name);
    HttpResponse::Ok().body("ok")
}

#[post("/function/{fun}")]
async fn sync_invoke(
    routes: web::Data<DashMap<String, String>>,
    req: web::Json<PublishRequest>,
    fun: web::Path<String>,
    pool: web::Data<Arc<NatsPool>>
) -> impl Responder {
    let sys_time = SystemTime::now();
    let function_name = fun.into_inner();
    let message = query(function_name.clone(), routes, req, pool).await.unwrap();
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time
                    .duration_since(sys_time)
                    .expect("Clock may have gone backwards");
    info!("request to function {} handled in {:?} ms", function_name, difference);
    HttpResponse::Ok().body(message.payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let nats_url: String = env::var("NATS_URL").unwrap_or("192.168.17.118:4222".to_string());
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let routes: Data<DashMap<String, String>> = Data::new(DashMap::new());
    let mut file = File::open("../config.json")?; // Open the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read file into a string

    let data: HashMap<String, String> = serde_json::from_str(&contents)?; // Parse JSON into HashMap
    println!("{:?}", data);
    for (key, value) in data {
        routes.insert(key, value);
    };

    //let nc = async_nats::connect(nats_url).await.unwrap();
    let nats_pool = create_nats_pool(&nats_url, 1000).await; // Pool with 10 connections
    let nats_pool = Arc::new(nats_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(routes.clone())
            .app_data(web::Data::new(nats_pool.clone()))
            .service(sync_invoke)
            .service(async_invoke)
            .wrap(middleware::Logger::default())
            
    })
    .workers(100)
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
