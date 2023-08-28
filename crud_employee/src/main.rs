use actix_web::{HttpServer, App, get, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World 123")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{App::new()
        .service(hello)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}