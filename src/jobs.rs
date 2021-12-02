use std::env;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Job {
    url: String,
    handler: String,
    id: u32,
    save_location: String,
    status: String,
}

fn get_url(path: &str) -> String {
    let base_url = env::var("BASE_URL").unwrap_or(String::from("http://localhost:5000"));
    return format!("{}/{}", base_url, path);
}

pub fn peek_job() -> Job {
    let client = reqwest::blocking::Client::new();
    let response = client.get(get_url("/job")).send().unwrap();
    let job: Job = response.json().unwrap();
    return job;
}

pub fn pop_job() -> Job {
    let client = reqwest::blocking::Client::new();
    let response = client.delete(get_url("/job")).send().unwrap();
    let job: Job = response.json().unwrap();
    return job;
}

pub fn update_job_status(job: &Job, status: &str) -> Job {
    let client = reqwest::blocking::Client::new();
    let job_with_status = Job {
        url: job.url.clone(),
        handler: job.handler.clone(),
        id: job.id,
        save_location: job.save_location.clone(),
        status: status.to_string(),
    };
    client.patch(get_url(format!("/job/{}", job.id).as_str())).json(&job_with_status).send().unwrap();
    return job_with_status;
}