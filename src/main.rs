use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Welcome to Rust Full-Stack App!")
}

#[get("/api/message")]
async fn api_message() -> impl Responder {
    HttpResponse::Ok().json("Hello from Actix!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(api_message)
    })
    .bind("127.0.0.1:9005")?
    .run()
    .await
}