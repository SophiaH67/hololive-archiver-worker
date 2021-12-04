use regex::Regex;
use std::process::Command;

// Accepts the URL to a youtube video or stream and returns the path to the downloaded file
// Returns a custom error if the video couldn't be downloaded
pub fn handle(url: &str) -> Result<&'static str, String> {
    loop {
        // Keep trying to download the video if the error contains "This live event will begin"
        // else return the error
        match attempt_download(url) {
            Ok(path) => return Ok(path),
            Err(e) => {
                let haystack = e.to_lowercase();

                if !haystack.contains("this live event will begin") {
                    if haystack.contains("no space left on device") {
                        return Err("No space left on device".to_string());
                    }
                    return Err(e);
                }

                // ERROR: This live event will begin in a few moments.
                // ERROR: This live event will begin in 40 minutes.
                // ERROR: This live event will begin in 5 hours.
                // ERROR: This live event will begin in 2 days.
                // ERROR: This live event will begin in 3 weeks.
                // ERROR: This live event will begin in 5 months.

                let number_regex = Regex::new(r"\d+").unwrap();
                let number_captures = number_regex.captures(&haystack).unwrap();
                let number = number_captures.get(0).unwrap().as_str().parse::<u64>().unwrap();

                println!("Got {} as time number from string {}", number, haystack);

                if haystack.contains("in a few moments") {
                    println!("ERROR: This live event will begin in a few moments. Sleeping for 10 seconds.");
                    std::thread::sleep(std::time::Duration::from_secs(10));
                } else if haystack.contains("minutes") {
                    println!("ERROR: This live event will begin in {} minutes. Sleeping for {} minutes.", number, number-1);
                    std::thread::sleep(std::time::Duration::from_secs((number-1) * 60));
                } else if haystack.contains("hours") {
                    println!("ERROR: This live event will begin in {} hours. Sleeping for {} hours.", number, number-1);
                    std::thread::sleep(std::time::Duration::from_secs((number-1) * 3600));
                } else if haystack.contains("days") {
                    println!("ERROR: This live event will begin in {} days. Sleeping for {} days.", number, number-1);
                    std::thread::sleep(std::time::Duration::from_secs((number-1) * 86400));
                } else if haystack.contains("weeks") {
                    println!("ERROR: This live event will begin in {} weeks. Sleeping for {} weeks.", number, number-1);
                    std::thread::sleep(std::time::Duration::from_secs((number-1) * 604800));
                } else if haystack.contains("months") {
                    println!("ERROR: This live event will begin in {} months. Sleeping for {} months.", number, number-1);
                    std::thread::sleep(std::time::Duration::from_secs((number-1) * 2592000));
                } else {
                    println!("ERROR: This live event will begin in unknown time.");
                    return Err(e);
                }
            }
        }
    }
}

fn attempt_download(url: &str) -> Result<&'static str, String> {
    let mut cmd = Command::new("yt-dlp");
    cmd.arg("-f");
    cmd.arg("best");
    cmd.arg("-o");
    cmd.arg("/tmp/ytdlp.mkv");
    cmd.arg(url);
    let output = cmd.output().unwrap();
    if output.status.success() {
        Ok("/tmp/ytdlp.mkv")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}
