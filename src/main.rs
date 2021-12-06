mod handlers;
mod jobs;
pub use handlers::yt_dlp;
pub use handlers::ytarchive;
use std::fs;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    loop {
        pop_and_run_job();
        sleep(Duration::from_secs(5));
    }
}

fn pop_and_run_job() {
    let mut job = match jobs::pop_job() {
        Ok(job) => job,
        Err(e) => {
            // Ignore 404's for now
            if e.to_string().contains("expected value at line 1 column 1") {
                return;
            }
            println!("Could not find a job to run, retrying in 5 seconds. {}", e);
            return;
        }
    };
    println!("Running job for: {:?}", job.url);
    job.update_status("running".to_string());
    job.update_hostname(gethostname::gethostname().to_str().unwrap().to_string());

    let handler = match job.handler.as_ref() {
        "yt-dlp" => yt_dlp::handle,
        "ytarchive" => ytarchive::handle,
        _ => panic!("Handler not found"),
    };

    let handler_result = handler(&job.url);
    println!("Creating folder {:?}", job.save_folder());
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
    println!("Moving file from {:?} to {:?}", tmp_file_path, job.save_folder());
    fs::copy(tmp_file_path, job.save_location.clone()).unwrap();
    fs::remove_file(tmp_file_path).unwrap();

    job.update_status("finished".to_string());
    println!("Finished job for: {:?}", job.url);
}