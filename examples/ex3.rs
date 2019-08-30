use actix_web::{web, App, HttpResponse, HttpServer};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}