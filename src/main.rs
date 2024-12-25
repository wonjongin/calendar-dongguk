mod crawler;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::{http, web};
use crawler::process_univs;
use serde::Serialize;
use std::env;

#[derive(Serialize, Debug)]
struct Schedule {
    at: Vec<String>,
    title: String,
    org: String,
}

#[get("/calendar/{filename}.ics")]
async fn get_calendar(path: web::Path<String>) -> impl Responder {
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

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Test")
}

#[get("/")]
async fn index() -> impl Responder {
    let html = match std::fs::read_dir("public") {
        Ok(entries) => {
            let mut calendars: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let name = e.file_name().to_string_lossy().to_string();
                        if name.starts_with("DONGGUK_") && name.ends_with(".ics") {
                            Some(name.trim_end_matches(".ics").to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect();
            calendars.sort();
            calendars.reverse();

            let links = calendars
                .iter()
                .map(|filename| {
                    let year = filename.trim_start_matches("DONGGUK_");
                    format!(
                        "<p><a href=\"/calendar/{}.ics\">동국대학교 {} 학사일정 다운로드</a></p>",
                        filename, year
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>동국대학교 학사일정 캘린더</title>
                    <meta charset="utf-8">
                </head>
                <body>
                    <h1>동국대학교 학사일정 캘린더</h1>
                    {}
                </body>
                </html>
            "#,
                links
            )
        }
        Err(_) => "Error reading calendar files".to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// async fn initialize_schedule() -> Result<(), Box<dyn std::error::Error>> {
//     // schedules.json 파일이 없을 때만 크롤링 실행
//     if !Path::new("schedules.json").exists() {
//         println!("schedules.json not found. Starting initial crawling...");
//         let url =
//             "https://www.dongguk.edu/schedule/detail?schedule_info_seq=22&schedule_start_date=2025";
//         let html = fetch_html(url).await?;
//         let schedules = parse_schedule(&html);
//         save_to_json(&schedules, "schedules.json")?;
//         println!("Schedule created at: {}", chrono::Local::now());
//     } else {
//         println!("schedules.json already exists. Skipping crawling.");
//     }
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if let Err(e) = process_univs().await {
        eprintln!("Error processing schools: {}", e);
    }

    #[cfg(debug_assertions)]
    const FRONT_PATH: &str = "./front/dist";

    #[cfg(not(debug_assertions))]
    const FRONT_PATH: &str = "./static";

    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    println!("Server running at http://localhost:{}", port);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            // .service(index)
            .service(test)
            .service(get_calendar)
            .service(fs::Files::new("/", FRONT_PATH).index_file("index.html"))
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
