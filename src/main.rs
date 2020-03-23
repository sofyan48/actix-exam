mod entity;

use entity::user;
use actix_web::{middleware, web, App, HttpServer, HttpResponse, HttpRequest};


async fn index_post_json(item: web::Json<user::MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0)
}

async fn index_json() -> HttpResponse {
    let obj = user::MyObj{name: String::from("Test"), number: 20};
    HttpResponse::Ok().json(obj)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/json").route(web::post().to(index_json)))
            .service(web::resource("/").route(web::get().to(index_post_json)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}