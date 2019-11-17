// src/main.rs
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind("127.0.0.1:5000")?
        .run()
}
