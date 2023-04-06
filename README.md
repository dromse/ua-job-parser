# 🇺🇦 UA JOB PARSER 

#### [🚧 in unstable-development stage]

Simple parser for gathering vacancies from ukranian job websites.

## Install

- in Cargo.toml
```
[dependencies]
tokio = "1.27.0"
ua-job-parser = "0.1.0"
```

- or use `cargo-add`
```bash
cargo add tokio ua-job-parser
```

## Usage

```rust
use ua_job_parser::{dou, Vacancy};

#[tokio::main]
async fn main() {
    // return list of vacancies from dou
    // if nothing was found, return empty list
    let vacancies: Vec<Vacancy> = dou::parse_vacancies("rust developer").await; 

    println!("Founded vacancies: {:#?}", vacancies);
}
```

## TODO

- [x] implement parser for dou
- [ ] implement parser for jooble
- [ ] implement parser for robota
- [ ] implement parser for work

## Contribution

I'll be glad if you take a look at my code and give me some advice or pull requests!
