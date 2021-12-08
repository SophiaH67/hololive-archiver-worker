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
        let error = Err(stderr.to_string());

        // Return custom error if "no space left on device"
        if stderr.contains("no space left on device") {
            return Err("No space left on device".to_string());
        }
        
        // Try yt-dlp if yt-archive error contains "Livestream has been processed. Use youtube-dl instead."
        if stderr.contains("youtube-dl") {
            return yt_dlp::handle(url);
        }

        error
    }
}
