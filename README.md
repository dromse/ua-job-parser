# 🇺🇦 UA JOB PARSER 

#### [🚧 in unstable-development stage]

Simple parser for gathering vacancies from ukranian job websites.

## Install

- in Cargo.toml
```
[dependencies]
tokio = "*"
ua-job-parser = "*"
```

- or use `cargo-add`
```bash
cargo add tokio ua-job-parser
```

## Usage

```rust
use ua_job_parser::{dou, robota, Vacancy};

#[tokio::main]
async fn main() {
    let query = "rust developer";

    // return list of vacancies
    // if nothing was found, return empty list
    let dou_vacancies: Vec<Vacancy> = dou::parse_vacancies(query).await; 
    let robota_vacancies: Vec<Vacancy> = robota::parse_vacancies(query).await; 

    println!("Founded vacancies from dou: {:#?}", dou_vacancies);
    println!("Founded vacancies from robota: {:#?}", robota_vacancies);
}
```

## TODO

- [x] implement parser for dou
- [x] implement parser for robota (upd. use api.robota.ua instead of parsing html)
- [ ] implement parser for work

## Contribution

I'll be glad if you take a look at my code and give me some advice or pull requests!
