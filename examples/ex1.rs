use actix_web::{web, App, HttpResponse, HttpServer, Responder};
fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(||HttpResponse::Ok().body("Hello world!")))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}