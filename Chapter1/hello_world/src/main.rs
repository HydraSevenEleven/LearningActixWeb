use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};

async fn users() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users</h1>"))
}

async fn login() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users -> Login</h1>"))
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users -> Logout</h1>"))
}

async fn register() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users -> Register</h1>"))
}

async fn profile() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users -> Profile</h1>"))
}

async fn settings() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1>Users -> Settings</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/users")
                .route("/", web::get().to(users))
                .route("/login", web::get().to(login))
                .route("/logout", web::get().to(logout))
                .route("/register", web::get().to(register))
                .route("/profile", web::get().to(profile))
                .route("/settings", web::get().to(settings))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}