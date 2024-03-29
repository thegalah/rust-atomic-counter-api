use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

async fn time() -> impl Responder {
    HttpResponse::Ok().body(Local::now().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/time", web::get().to(time))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
