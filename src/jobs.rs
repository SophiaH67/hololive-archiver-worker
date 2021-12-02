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

fn get_url(path: &str) -> &str{
    let base_url = env::var("BASE_URL").unwrap_or(String::from("http://localhost:5000"));
    return format!("{}/{}", base_url, path);
}

pub fn peek_job() {
    let client = reqwest::blocking::Client::new();
    let response = client.get(get_url("/job")).send().unwrap();
    let job: Job = response.json().unwrap();
    println!("{:?}", job);
}
