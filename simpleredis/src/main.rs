
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

async fn set_value(
    data: web::Json<KeyValue>,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    let con_result = redis_client.get_multiplexed_async_connection().await;

    let mut con = match con_result {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Redis error: {}", e)),
    };

    // ✅ Set key-value in Redis
    let result: redis::RedisResult<()> = con.set(&data.key, &data.value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Value set"),
        Err(e) => HttpResponse::InternalServerError().body(format!("SET error: {}", e)),
    }
}

async fn get_value(
    key: web::Path<String>,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    let con_result = redis_client.get_multiplexed_async_connection().await;

    let mut con = match con_result {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Redis error: {}", e)),
    };

    let result: redis::RedisResult<String> = con.get(&*key).await;
    match result {
        Ok(val) => HttpResponse::Ok().body(val),
        Err(_) => HttpResponse::NotFound().body("Key not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect Redis");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .route("/set", web::post().to(set_value))
            .route("/get/{key}", web::get().to(get_value))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
