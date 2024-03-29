use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Handler for the /health endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server operational")
}

// Handler for the /liveness endpoint
async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("Server responsive")
}

// Handler for the /counter endpoint
async fn counter(counter: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let count_value = counter.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok().json(serde_json::json!({ "counter": count_value }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .route("/health", web::get().to(health))
            .route("/liveness", web::get().to(liveness))
            .route("/counter", web::get().to(counter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
