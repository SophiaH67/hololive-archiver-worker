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
    let mut job = jobs::pop_job();
    println!("Running job for: {:?}", job.url);
    job.update_status("running".to_string());

    let handler = match job.handler.as_ref() {
        "yt-dlp" => yt_dlp::handle,
        _ => panic!("Handler not found"),
    };

    let tmp_file = handler(&job.url);
    fs::create_dir_all(job.save_folder()).unwrap();
    let tmp_file_path = tmp_file.unwrap();
    fs::copy(tmp_file_path, job.save_location.clone()).unwrap();
    fs::remove_file(tmp_file_path).unwrap();

    job.update_status("finished".to_string());
    println!("Finished job for: {:?}", job.url);
}