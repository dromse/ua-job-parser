#[derive(Debug)]
pub struct Website {
    pub name: String,
    pub link: String,
    pub query: String,
    pub total: u16,
    pub vacancies: Vec<Vacancy>,
}

#[derive(Debug)]
pub struct Vacancy {
    pub position: String,
    pub company: String,
    pub description: String,
    pub date: String,
    pub location: String,
    pub salary: String,
    pub link: String,
}

pub mod dou;
