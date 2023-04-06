use super::*;
use log::{error, info, warn};
use reqwest::header::{HeaderMap, HeaderValue};
use scraper::{Html, Selector};

const VACANCIES_PER_REQUEST: u16 = 40;
const LINK: &str = "https://jobs.dou.ua/vacancies/?search=";

pub async fn parse_vacancies(query: &str) -> Vec<Vacancy> {
    env_logger::init();

    let link = format!("{LINK}{query}");

    let response = reqwest::get(&link).await;

    let html_response_headers = response.as_ref().unwrap().headers().clone();

    let cookie = html_response_headers
        .get("set-cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .split(';')
        .collect::<Vec<&str>>()[0];

    let csrf_middleware_token = cookie.split('=').collect::<Vec<&str>>()[1];

    let mut headers = HeaderMap::new();
    headers.insert("Referer", HeaderValue::from_str(link.as_str()).unwrap());
    headers.insert("Cookie", HeaderValue::from_str(cookie).unwrap());

    let client = reqwest::Client::builder()
        .referer(true)
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .unwrap();

    let html_document = response.unwrap().text().await.unwrap();

    let html_fragment = Html::parse_fragment(&html_document);

    let found_vacancies = get_total_amount_of_vacancies(&html_fragment).await;

    let total_shifts = get_total_shifts(found_vacancies).await;

    // selectors for parsing data from list of vacancies on website
    let vacancy_selector = Selector::parse("li.l-vacancy .vacancy").unwrap();
    let position_selector = Selector::parse(".title a.vt").unwrap();
    let company_selector = Selector::parse(".title a.company").unwrap();
    let description_selector = Selector::parse(".sh-info").unwrap();
    let salary_selector = Selector::parse(".title span.salary").unwrap();
    let location_selector = Selector::parse(".title span.cities").unwrap();
    let date_selector = Selector::parse(".date").unwrap();
    let link_selector = Selector::parse(".title a.vt").unwrap();

    // init storage for vacancies
    let mut vacancies: Vec<Vacancy> = Vec::new();

    for shift in 0..=total_shifts {
        let offset = calculate_offset(shift).await;

        // params for getting new amount of vacancies
        let params = [
            ("csrfmiddlewaretoken", csrf_middleware_token),
            ("count", offset.as_str()),
        ];

        let res = client
            .post(format!(
                "https://jobs.dou.ua/vacancies/xhr-load/?search={query}"
            ))
            .form(&params)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let ajax_body = res["html"].as_str().unwrap();

        let ajax_fragment = Html::parse_fragment(ajax_body);

        // parse vacancies from ajax response
        for ajax_element in ajax_fragment.select(&vacancy_selector) {
            let position = ajax_element
                .select(&position_selector)
                .next()
                .unwrap()
                .inner_html()
                .replace("&nbsp;", " ");

            let company = Html::parse_fragment(
                &ajax_element
                    .select(&company_selector)
                    .next()
                    .unwrap()
                    .html(),
            )
            .select(&Selector::parse(".company").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
                .to_string()
                .replace("\u{a0}", "");

            let link = ajax_element
                .select(&link_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();

            let description;
            let mut element_description =
                ajax_element.select(&description_selector);

            if element_description.clone().count() == 0 {
                warn!(
                    "Description for '{position}' from '{company}' wasn't \
                     found."
                );
                description = String::new();
            } else {
                description = element_description
                    .next()
                    .unwrap()
                    .inner_html()
                    .trim()
                    .to_string()
                    .replace("&nbsp;", " ")
                    .replace("\n\n", " ");
            }

            let salary;
            let mut element_salary = ajax_element.select(&salary_selector);

            if element_salary.clone().count() == 0 {
                warn!(
                    "Salary for '{position}' from '{company}' wasn't found."
                );
                salary = String::new();
            } else {
                salary = element_salary.next().unwrap().inner_html();
            }

            let date;
            let mut element_date = ajax_element.select(&date_selector);

            if element_date.clone().count() == 0 {
                warn!("Date for '{position}' from '{company}' wasn't found.");
                date = String::new();
            } else {
                date = element_date.next().unwrap().inner_html();
            }

            let location;
            let mut element_location = ajax_element.select(&location_selector);

            if element_location.clone().count() == 0 {
                warn!(
                    "Location for '{position}' from '{company}' wasn't found."
                );

                location = String::new();
            } else {
                location = element_location.next().unwrap().inner_html();
            }

            vacancies.push(Vacancy {
                position,
                company,
                description,
                salary,
                link,
                date,
                location,
            });
        }
    }

    vacancies
}

async fn get_total_amount_of_vacancies(fragment: &Html) -> u16 {
    let selector_found_vacancies =
        Selector::parse(".b-inner-page-header h1").unwrap();

    let mut title_found_vacancies = fragment
        .select(&selector_found_vacancies)
        .next()
        .unwrap()
        .inner_html();

    let first_space = title_found_vacancies.find(' ').unwrap();

    title_found_vacancies.truncate(first_space);

    let found_vacancies = match title_found_vacancies.parse::<u16>() {
        Ok(number) => number,
        Err(_) => {
            error!("Vacancies not found! Return zero value.");
            return 0;
        }
    };

    info!("Found total vacancies from title: {}", found_vacancies);

    found_vacancies
}

async fn get_total_shifts(total_vacancies: u16) -> u16 {
    total_vacancies / VACANCIES_PER_REQUEST
}

async fn calculate_offset(shift: u16) -> String {
    (shift as f64 * VACANCIES_PER_REQUEST as f64)
        .ceil()
        .to_string()
}
