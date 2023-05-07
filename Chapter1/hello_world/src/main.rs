use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Hello world!</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}