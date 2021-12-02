use std::env;
use serde::{Deserialize};

#[derive(Deserialize)]
#[derive(Debug)]
struct Job {
    url: String,
    handler: String,
    id: u32,
    save_location: String,
    status: String,
}

pub fn peek_job() {
    let client = reqwest::blocking::Client::new();
    // Read base_url from environment variable BASE_URL or use default value
    let base_url = env::var("BASE_URL").unwrap_or(String::from("http://localhost:5000"));

    let url = format!("{}/job", base_url);
    let response = client.get(&url).send().unwrap();
    let job: Job = response.json().unwrap();
    println!("{:?}", job);
}
