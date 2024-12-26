use std::path::Path;

use actix_web::{get, web};
use actix_web::{HttpResponse, Responder};

use crate::crawler::json_file_to_ics;
use crate::crawler::univ_config::UnivConfig;

#[get("/calendar/{filename}.ics")]
pub async fn get_calendar(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let file_path = format!("public/{}.ics", filename);

    match std::fs::read_to_string(&file_path) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/calendar")
            .append_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}.ics\"", filename),
            ))
            .body(content),
        Err(_) => HttpResponse::NotFound().body("Calendar file not found"),
    }
}

#[get("/c/{univ_code}_{year_code}_{hash}/cal.ics")]
pub async fn get_calendar_by_short(path: web::Path<(String, String, u8)>) -> impl Responder {
    let (univ_code, year_code, hash) = path.into_inner();
    let file_path = format!("public/{}_{}_{}.ics", univ_code, year_code, hash);

    if let Some(univ_config) = UnivConfig::get_by_code(&univ_code) {
        let year_config = univ_config.years.iter().find(|y| y.year_code == year_code);
        if year_config.is_none() {
            return HttpResponse::NotFound().body("Year not found");
        }
        let year = year_config.unwrap().year;
        let prefix = univ_config.prefix.as_str();

        if Path::new(&file_path).exists() {
            println!("{} 파일이 존재합니다", file_path);
        } else {
            json_file_to_ics(
                format!("data/{}_{}_classified.json", prefix, year).as_str(),
                format!("public/{}_{}_{}.ics", univ_code, year_code, hash).as_str(),
                &univ_config.name,
                year,
                hash,
            );
        }

        match std::fs::read_to_string(&file_path) {
            Ok(content) => HttpResponse::Ok()
                .content_type("text/calendar")
                .append_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}.ics\"", "cal"),
                ))
                .body(content),
            Err(_) => HttpResponse::NotFound().body("Calendar file not found"),
        }
    } else {
        return HttpResponse::NotFound().body("University not found");
    }
}
