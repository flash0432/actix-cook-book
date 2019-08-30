use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i64>,
}

#[derive(Deserialize)]
struct Name {
    #[serde(default = "Name::name_default_value")]
    name: String,
}
impl Name {
    fn name_default_value() -> String {
        String::from("World!")
    }
}

#[derive(Serialize)]
struct Greeting {
    id: i64,
    content: String,
}

#[get("/greeting")]
fn index(data: web::Data<AppState>, query: web::Query<Name>) -> impl Responder {
    let query = &query.name;
    let mut cnt = data.counter.lock().unwrap();
    *cnt += 1;

    HttpResponse::Ok().json(Greeting {
        id: *cnt,
        content: format!("hello, {}", query),
    })
}

fn main() {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || App::new().register_data(app_state.clone()).service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap();
}
