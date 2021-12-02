use std::process::Command;

// Accepts the URL to a youtube video or stream and returns the path to the downloaded file
// Returns a custom error if the video couldn't be downloaded
pub fn handle(url: &str) -> Result<&'static str, String> {
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
