use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct YearConfig {
    pub year: i32,
    pub year_code: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct UnivConfig {
    pub name: String,
    pub prefix: String,
    pub univ_code: String,
    pub years: Vec<YearConfig>,
    pub crawler_type: String,
}

impl UnivConfig {
    pub fn get_by_code(univ_code: &str) -> Option<&'static UnivConfig> {
        UNIV_CONFIGS.get(univ_code)
    }

    pub fn get_name_by_code(univ_code: &str) -> Option<&'static str> {
        UNIV_CONFIGS
            .get(univ_code)
            .map(|config| config.name.as_str())
    }

    pub fn get_year_by_code(year_code: &str) -> Option<&'static str> {
        UNIV_CONFIGS
            .values()
            .flat_map(|config| config.years.iter())
            .find(|year| year.year_code == year_code)
            .map(|year| year.url.as_str())
    }
}

lazy_static! {
    static ref UNIV_CONFIGS: HashMap<String, UnivConfig> = {
        let config_str = include_str!("../../univ.json");
        let configs: Vec<UnivConfig> = serde_json::from_str(config_str).unwrap();
        configs
            .into_iter()
            .map(|config| (config.univ_code.clone(), config))
            .collect()
    };
}
