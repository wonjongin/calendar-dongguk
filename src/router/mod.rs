use actix_web::{get, HttpResponse, Responder};

pub mod calendar;

#[get("/univ.json")]
pub async fn get_univ_config() -> impl Responder {
    match std::fs::read_to_string("univ.json") {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/json")
            .body(content),
        Err(_) => HttpResponse::NotFound().body("univ.json file not found"),
    }
}
