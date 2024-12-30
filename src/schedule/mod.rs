use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Schedule {
    pub at: Vec<String>,
    pub title: String,
    pub org: String,
    pub category: String, // 카테고리 필드 추가
}

pub struct ScheduleCategoryList {
    pub categories: Vec<String>,
}

impl ScheduleCategoryList {
    pub fn from_categories(schedule_categories: Vec<String>) -> ScheduleCategoryList {
        ScheduleCategoryList {
            categories: schedule_categories,
        }
    }

    pub fn all() -> ScheduleCategoryList {
        ScheduleCategoryList {
            categories: vec![
                "수업".to_string(),
                "학적".to_string(),
                "대학원".to_string(),
                "장학".to_string(),
                "교직".to_string(),
                "행사".to_string(),
                "기타".to_string(),
                "졸업".to_string(),
            ],
        }
    }

    pub fn to_hash(&self) -> u32 {
        let all_categories = Self::all();
        let mut hash: u32 = 0;

        for category in &self.categories {
            if let Some(index) = all_categories.categories.iter().position(|c| c == category) {
                hash |= 1 << index;
            }
        }
        hash
    }

    pub fn from_hash(hash: u32) -> ScheduleCategoryList {
        let all_categories = Self::all();
        let mut selected_categories = Vec::new();

        for (index, category) in all_categories.categories.iter().enumerate() {
            if (hash & (1 << index)) != 0 {
                selected_categories.push(category.clone());
            }
        }

        ScheduleCategoryList {
            categories: selected_categories,
        }
    }

    pub fn contains(&self, category: &str) -> bool {
        self.categories.iter().any(|c| c == category)
    }
}
