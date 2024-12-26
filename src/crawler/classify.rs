use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

use crate::schedule::Schedule;

fn categorize_schedules(schedules: &mut Vec<Schedule>) {
    let keywords = HashMap::from([
        (
            "수업".to_string(),
            vec![
                "수강신청",
                "수강 신청",
                "시험",
                "계절학기",
                "개강",
                "종강",
                "보강",
                "기준일",
                "성적처리",
            ],
        ),
        (
            "학적".to_string(),
            vec![
                "휴학",
                "복학",
                "전과",
                "다전공",
                "재입학",
                "졸업연기",
                "전공신청",
                "졸업대상자",
                "조기졸업",
                "등록",
                "학점포기",
            ],
        ),
        (
            "대학원".to_string(),
            vec!["대학원", "논문", "석사", "학위", "박사"],
        ),
        ("장학".to_string(), vec!["장학"]),
        (
            "교직".to_string(),
            vec!["교직", "교육실습", "교육봉사", "교원", "학교현장실습"],
        ),
        (
            "행사".to_string(),
            vec!["입학식", "학위수여식", "방학", "기념", "부처님"],
        ),
    ]);

    for schedule in schedules.iter_mut() {
        let title = &schedule.title;

        schedule.category = keywords
            .iter()
            .find(|(_, words)| words.iter().any(|&word| title.contains(word)))
            .map(|(category, _)| category.clone())
            .unwrap_or_else(|| "기타".to_string());
    }
}

fn save_schedules(schedules: &[Schedule], filename: &str) -> std::io::Result<()> {
    // 전체 일정을 하나의 JSON 파일로 저장
    let json = serde_json::to_string_pretty(&schedules)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;

    // 카테고리별 통계 출력
    let mut category_counts = std::collections::HashMap::new();
    for schedule in schedules {
        *category_counts.entry(&schedule.category).or_insert(0) += 1;
    }

    println!("\n=== 카테고리별 일정 수 ===");
    for (category, count) in category_counts {
        println!("{}: {} 항목", category, count);
    }

    println!("\n전체 일정이 {} 파일에 저장되었습니다.", filename);
    Ok(())
}

fn read_json_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn classify(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let json_str = match read_json_file(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("파일 읽기 실패: {}", e);
            return Err(e);
        }
    };
    let mut schedules: Vec<Schedule> = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("JSON 파싱 실패: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
        }
    };

    categorize_schedules(&mut schedules);
    save_schedules(&schedules, output_file)?;

    Ok(())
}
