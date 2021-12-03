use std::env;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Job {
    pub url: String,
    pub handler: String,
    pub id: u32,
    pub save_location: String,
    pub status: String,
    pub error: String,
}

impl Job {
    pub fn save_folder(&self) -> String {
        let path = PathBuf::from(self.save_location.clone());
        let mut folder = path.parent().unwrap().to_str().unwrap().to_string();
        folder.push_str("/");
        folder
    }

    pub fn update_status(&mut self, status: String) {
        self.status = status;
        let client = reqwest::blocking::Client::new();
        client.patch(get_url(format!("/job/{}", self.id).as_str())).json(&self).send().unwrap();
    }

    pub fn update_error(&mut self, error: String) {
        self.error = error;
        let client = reqwest::blocking::Client::new();
        client.patch(get_url(format!("/job/{}", self.id).as_str())).json(&self).send().unwrap();
    }
}

fn get_url(path: &str) -> String {
    let base_url = env::var("BASE_URL").unwrap_or(String::from("http://localhost:5000"));
    return format!("{}/{}", base_url, path);
}

pub fn pop_job() -> Result<Job, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.delete(get_url("/job")).send().unwrap();
    return response.json();
}
