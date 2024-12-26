use scraper::{Html, Selector};

use anyhow::Result;
use async_trait::async_trait;

use crate::schedule::Schedule;

use super::{fetch_html, parse_date, ScheduleCrawler};

pub struct DonggukCrawler;

#[async_trait]
impl ScheduleCrawler for DonggukCrawler {
    async fn crawl(&self, url: &str) -> Result<Vec<Schedule>> {
        let html = fetch_html(url).await?;
        let schedules = parse_schedule(&html);
        // save_to_json(&schedules, "schedules.json")?;
        Ok(schedules)
    }
}

fn parse_schedule(html: &str) -> Vec<Schedule> {
    let document = Html::parse_document(html);
    let table_selector =
        Selector::parse("#content_focus > div > div.cont_group > div.schedule > div > table")
            .unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut schedules = Vec::new();

    if let Some(table) = document.select(&table_selector).next() {
        for tr in table.select(&tr_selector) {
            let tds: Vec<_> = tr.select(&td_selector).collect();
            if tds.len() >= 2 {
                let date_text = tds[0].text().collect::<String>().trim().to_string();
                let content_text = tds[1].text().collect::<String>().trim().to_string();

                let mut at = Vec::new();
                if date_text.contains('~') {
                    let dates: Vec<&str> = date_text.split('~').collect();
                    at.push(parse_date(dates[0]));
                    at.push(parse_date(dates[1]));
                } else {
                    at.push(parse_date(&date_text));
                }

                let mut title = content_text.clone();
                let mut org = String::new();

                if let Some(idx) = content_text.find("(주관부서:") {
                    title = content_text[..idx].trim().to_string();
                    org = content_text[idx..]
                        .trim_start_matches("(주관부서:")
                        .trim_end_matches(')')
                        .trim()
                        .to_string();
                }

                schedules.push(Schedule {
                    at,
                    title,
                    org,
                    category: "".to_string(),
                });
            }
        }
    }

    schedules
}
