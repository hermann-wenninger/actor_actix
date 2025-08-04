use actix_web::{ web, App, HttpServer, Responder, HttpResponse, HttpRequest};
// connect to static files?
use rust_embed::RustEmbed;

async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html")
                      .body(include_str!("../index.html"))
}