mod crawler;
mod router;
mod schedule;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{http, web};
use actix_web::{App, HttpServer};
use crawler::process_univs;
use router::calendar::{get_calendar, get_calendar_by_short};
use router::get_univ_config;
use std::env;

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
            .service(get_calendar)
            .service(get_calendar_by_short)
            .service(get_univ_config)
            .service(
                fs::Files::new("/", FRONT_PATH)
                    .index_file("index.html")
                    .default_handler(web::get().to(|| async {
                        fs::NamedFile::open_async(format!("{}/index.html", FRONT_PATH)).await
                    })),
            )
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
