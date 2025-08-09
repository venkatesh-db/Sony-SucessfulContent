
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use scylla::{Session, SessionBuilder};
use std::sync::Arc;

#[get("/get")]
async fn get_data(session: web::Data<Arc<Session>>) -> impl Responder {
    // Query the value from myks.users
    let rows = session
        .query("SELECT name FROM myks.users LIMIT 1", &[])
        .await;

    match rows {
        Ok(res) => {
            if let Some(row) = res.rows.and_then(|mut r| r.pop()) {
                let name: String = row.columns[0].as_ref().unwrap().as_text().unwrap().to_string();
                HttpResponse::Ok().body(format!("Value: {}", name))
            } else {
                HttpResponse::NotFound().body("No data found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to ScyllaDB
    let session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");

    // Create Keyspace and Table
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS myks WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};",
            &[],
        )
        .await
        .unwrap();

    session
        .query(
            "CREATE TABLE IF NOT EXISTS myks.users (name TEXT PRIMARY KEY);",
            &[],
        )
        .await
        .unwrap();

    // Insert Data
    session
        .query("INSERT INTO myks.users (name) VALUES ('Venkatesh');", &[])
        .await
        .unwrap();

    // Share the session
    let session_data = web::Data::new(Arc::new(session));

    println!("Server running at http://127.0.0.1:9080/");
    HttpServer::new(move || {
        App::new()
            .app_data(session_data.clone())
            .service(get_data)
    })
    .bind(("127.0.0.1", 9080))?
    .run()
    .await
}


