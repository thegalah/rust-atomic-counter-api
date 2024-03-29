use actix_web::web::block;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server operational")
}

async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("Server responsive")
}

// Helper function to write the counter value to a file
fn write_counter_to_file(count_value: usize) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("counter.txt")?;
    writeln!(file, "{}", count_value)?;
    Ok(())
}

// Helper function to read the initial counter value from a file
fn read_initial_counter_value() -> usize {
    let path = "counter.txt";
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(last_value) = contents.lines().last().unwrap_or("0").parse::<usize>() {
            return last_value;
        }
    }
    0
}

async fn counter(counter: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let count_value = counter.fetch_add(1, Ordering::SeqCst);

    // Execute the blocking file write operation asynchronously
    let write_result = block(move || write_counter_to_file(count_value)).await;

    match write_result {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("{}", count_value)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read initial counter value from file
    let initial_counter_value = read_initial_counter_value();
    let counter_data = web::Data::new(Arc::new(AtomicUsize::new(initial_counter_value)));

    HttpServer::new(move || {
        App::new()
            .app_data(counter_data.clone())
            .route("/health", web::get().to(health))
            .route("/liveness", web::get().to(liveness))
            .route("/counter", web::get().to(counter))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
