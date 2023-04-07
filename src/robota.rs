use super::*;
use serde::{Deserialize, Serialize};

const VACANCIES_PER_REQUEST: u8 = 59;
const API_LINK: &str = "https://api.rabota.ua/vacancy/search?keyWords=";

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RobotaVacancy {
    name: String,
    cityName: String,
    companyName: String,
    date: String,
    shortDescription: String,
    salary: u32,
    notebookId: u32,
    id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiResponse {
    total: u16,
    documents: Vec<RobotaVacancy>,
}

async fn get_total_pages(total_amount_vacancies: u16) -> u16 {
    (total_amount_vacancies as f64 / VACANCIES_PER_REQUEST as f64).ceil()
        as u16
}

pub async fn parse_vacancies(query: &str) -> Vec<Vacancy> {
    let count_param = format!("&count={}", VACANCIES_PER_REQUEST);

    let mut query_link = format!("{API_LINK}{query}{count_param}");

    let total_amount_vacancies = reqwest::get(&query_link)
        .await
        .unwrap()
        .json::<ApiResponse>()
        .await
        .unwrap()
        .total;

    let pages = get_total_pages(total_amount_vacancies).await;

    let mut vacancies: Vec<Vacancy> = Vec::new();

    for page in 0..pages {
        let page_param = format!("&page={}", page);

        query_link = format!("{API_LINK}{query}{count_param}{page_param}");

        let vacancies_json = reqwest::get(&query_link)
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
            .unwrap();

        for vacancy_json in vacancies_json.clone().documents {
            vacancies.push(Vacancy {
                position: vacancy_json.name,
                company: vacancy_json.companyName,
                description: vacancy_json
                    .shortDescription
                    .trim()
                    .to_string()
                    .replace("\u{a0}", " "),
                salary: vacancy_json.salary.to_string(),
                link: format!(
                "https://rabota.ua/ua/company{company_id}/vacancy{vacancy_id}",
                company_id = vacancy_json.notebookId,
                vacancy_id = vacancy_json.id
            ),
                date: vacancy_json.date.split('T').collect::<Vec<&str>>()[0]
                    .to_string(),
                location: vacancy_json.cityName,
            });
        }
    }

    vacancies
}
