use chrono::{DateTime, Local};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    loop {
        // Ping google.com
        let response = Client::new().get("http://google.com").send();

        // Check internet connectivity
        let timestamp = get_current_timestamp();
        match response.await {
            Ok(_) => {
                // Internet connection is available
                // Do something here or simply print a success message
                // println!("Internet connection is available at {}", timestamp);
            }
            Err(_) => {
                // Append logs with date and time stamp
                let log = String::from("No internet connection at");
                println!("{} {}", log, timestamp);
                append_log(format!("{} {}", log, timestamp));
            }
        }

        // Wait for 5 seconds before pinging again
        thread::sleep(Duration::from_secs(2));
    }
}

fn get_current_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn append_log(message: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Failed to write to log file: {}", e);
    }
}
