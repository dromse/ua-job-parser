# 🇺🇦 UA JOB PARSER 
#### [🚧 in unstable-development stage]

**ua-job-parser** is a simple parser on rust for gathering vacancies from:

- dou.ua

## 🛠️ Usage

```rust
use ua_job_parser::{dou, Vacancy}

#[tokio::main]
fn main() {
    // return list of vacancies from dou
    // if nothing was found, return empty list
    let vacancies: Vec<Vacancy> = dou::parse_vacancies("rust developer").await.unwrap(); 

    println!("Founded vacancies: {}", vacancies);
}
```

## ✅ TODO

- [x] implement parser for dou
- [ ] implement parser for jooble
- [ ] implement parser for robota
- [ ] implement parser for work

## 🧨 Contribution

I'll be glad if you take a look at my code and give me some advice or pull requests!
