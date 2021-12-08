use regex::Regex;
use std::process::Command;
use crate::yt_dlp;

// Accepts the URL to a youtube video or stream and returns the path to the downloaded file
// Returns a custom error if the video couldn't be downloaded
pub fn handle(url: &str) -> Result<&'static str, String> {
    let mut cmd = Command::new("ytarchive");
    cmd.arg("--add-metadata");
    cmd.arg("-o");
    cmd.arg("/tmp/ytarchive.mkv");
    cmd.arg("-w");
    cmd.arg("--mkv");
    cmd.arg(url);
    cmd.arg("best");
    let output = cmd.output().unwrap();
    if output.status.success() {
        Ok("/tmp/ytarchive.mkv")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Using regex, check if "no space left on device" is anywhere in the stderr ignoring case
        let space_regex = Regex::new(r"no space left on device").unwrap();
        if space_regex.is_match(&stderr) {
            return Err(String::from("No space left on device"));
        }
        
        // Try yt-dlp if yt-archive error contains "youtube-dl"
        let ytdlp_regex = Regex::new(r"youtube-dl").unwrap();
        if ytdlp_regex.is_match(&stderr) {
            return yt_dlp::handle(url);
        }

        // Return stdout if stderr is empty
        if stderr.len() == 0 {
            return Err(String::from_utf8_lossy(&output.stdout).to_string());
        }
        return Err(stderr.to_string());
    }
}
