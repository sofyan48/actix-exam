use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

// #[get("/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

async fn index_json() -> HttpResponse {
    let obj = MyObj{name: String::from("Test"), number: 20};
    HttpResponse::Ok().json(obj)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/json").route(web::get().to(index_json)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}