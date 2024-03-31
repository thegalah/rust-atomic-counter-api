use actix_web::web::block;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::env;
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

fn write_counter_to_file(count_value: usize) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("counter.txt")?;
    writeln!(file, "{}", count_value)?;
    Ok(())
}

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
    println!("RUST_LOG: {:?}", env::var("RUST_LOG").ok());
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let initial_counter_value = read_initial_counter_value();
    let counter_data = web::Data::new(Arc::new(AtomicUsize::new(initial_counter_value)));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    let host = "0.0.0.0";

    log::info!("Server running on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Add the Logger middleware
            .app_data(counter_data.clone())
            .route("/health", web::get().to(health))
            .route("/liveness", web::get().to(liveness))
            .route("/counter", web::get().to(counter))
    })
    .bind((host, port))?
    .run()
    .await
}
