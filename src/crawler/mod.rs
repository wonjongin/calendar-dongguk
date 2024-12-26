pub mod classify;
mod dongguk;
pub mod univ_config;

use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use dongguk::DonggukCrawler;
use ics::parameters::Value;
use ics::properties::{Description, DtEnd, DtStart, LastModified, Status, Summary, TzName};
use ics::{escape_text, Event, ICalendar, Standard, TimeZone};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use univ_config::UnivConfig;

use crate::schedule::{Schedule, ScheduleCategoryList};

// 크롤링 함수들을 trait로 정의
#[async_trait]
trait ScheduleCrawler {
    async fn crawl(&self, url: &str) -> Result<Vec<Schedule>>;
}

fn get_crawler(crawler_type: &str) -> Box<dyn ScheduleCrawler> {
    match crawler_type {
        "dongguk" => Box::new(DonggukCrawler),
        _ => panic!("Unknown crawler type: {}", crawler_type),
    }
}

pub async fn process_univs() -> Result<()> {
    let file = File::open("univ.json")?;
    let univs: Vec<UnivConfig> = serde_json::from_reader(file)?;

    for univ in univs {
        let crawler = get_crawler(&univ.crawler_type);

        for year_config in univ.years {
            let filename = format!("public/{}_{}.ics", univ.prefix, year_config.year);
            let filename_json = format!("data/{}_{}.json", univ.prefix, year_config.year);
            if Path::new(&filename).exists() {
                println!("{} 파일이 존재합니다", filename);
            } else {
                match crawler.crawl(&year_config.url).await {
                    Ok(schedules) => {
                        create_ics(&schedules, &filename, &univ.name, year_config.year);
                        save_to_json(&schedules, &filename_json)?;
                        // std::fs::write(filename, calendar)?;
                        println!(
                            "Successfully created calendar for {} {}",
                            univ.name, year_config.year
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to crawl {} schedule for year {}: {}",
                            univ.name, year_config.year, e
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?;

    response.text().await
}

pub fn parse_date(date_str: &str) -> String {
    date_str.trim().trim_end_matches('.').replace('.', "-")
}

pub fn save_to_json(schedules: &[Schedule], filename: &str) -> std::io::Result<()> {
    // JSON 문자열로 변환
    let json_string = serde_json::to_string_pretty(schedules)?;

    // 파일 생성 및 쓰기
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;

    println!("Successfully saved to {}", filename);
    Ok(())
}

pub fn json_file_to_ics(
    json_file: &str,
    ics_file: &str,
    univ: &str,
    year: i32,
    hash: u8,
) -> String {
    let json_str = match std::fs::read_to_string(json_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return "".to_string();
        }
    };
    let schedules: Vec<Schedule> = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return "".to_string();
        }
    };
    let schedule_category_list = ScheduleCategoryList::from_hash(hash);

    let filtered_schedules: Vec<Schedule> = schedules
        .into_iter()
        .filter(|schedule| schedule_category_list.contains(&schedule.category))
        .collect();

    create_ics(&filtered_schedules, &ics_file, univ, year)
}

pub fn create_ics(schedules: &[Schedule], filename: &str, univ: &str, year: i32) -> String {
    let mut calendar = ICalendar::new("2.0", format! {"-//{} {} 학사일정//KR", univ, year});
    let mut standard = Standard::new("19700101T000000", "+0900", "+0900");
    standard.push(TzName::new("KST"));

    let mut timezone = TimeZone::standard("Asia/Seoul", standard);
    timezone.push(LastModified::new("20230615T050000Z"));
    calendar.add_timezone(timezone);

    for schedule in schedules {
        let mut event = Event::new(
            uuid::Uuid::new_v4().to_string(),
            chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string(),
        );
        let start_date = NaiveDate::parse_from_str(&schedule.at[0], "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

        let end_date = if schedule.at.len() > 1 {
            NaiveDate::parse_from_str(&schedule.at[1], "%Y-%m-%d").unwrap_or(start_date)
        } else {
            start_date
        };
        let mut dtstart = DtStart::new(start_date.format("%Y%m%d").to_string());
        dtstart.add(Value::DATE);
        let mut dtend = DtEnd::new(end_date.format("%Y%m%d").to_string());
        dtend.add(Value::DATE);

        event.push(dtstart);
        event.push(dtend);
        event.push(Summary::new(&schedule.title));
        if !schedule.org.is_empty() {
            event.push(Description::new(escape_text(format!(
                "{} 관련\n주관부서: {}",
                schedule.category, schedule.org
            ))));
        }
        event.push(Status::confirmed());

        calendar.add_event(event);
    }
    if let Err(e) = calendar.save_file(filename) {
        eprintln!("Failed to save ICS file: {}", e);
    }
    calendar.to_string()
}
