mod handlers;
mod jobs;
pub use handlers::yt_dlp;
use std::fs;

fn main() {
    loop {
        pop_and_run_job();
    }
}

fn pop_and_run_job() {
    let mut job = match jobs::pop_job() {
        Ok(job) => job,
        Err(e) => {
            println!("Could not find a job to run, retrying in 5 seconds. Error: {}", e);
            return;
        }
    };
    println!("Running job for: {:?}", job.url);
    job.update_status("running".to_string());

    let handler = match job.handler.as_ref() {
        "yt-dlp" => yt_dlp::handle,
        _ => panic!("Handler not found"),
    };

    let handler_result = handler(&job.url);
    fs::create_dir_all(job.save_folder()).unwrap();
    let tmp_file_path = match handler_result {
        Ok(tmp_file_path) => tmp_file_path,
        Err(e) => {
            println!("Error: {:?}", e);
            job.update_status("error".to_string());
            job.update_error(e.to_string());
            return;
        }
    };
    fs::copy(tmp_file_path, job.save_location.clone()).unwrap();
    fs::remove_file(tmp_file_path).unwrap();

    job.update_status("finished".to_string());
    println!("Finished job for: {:?}", job.url);
}