use std::{env, fs::OpenOptions, io::Write, thread, time::Duration};
use chrono::Utc;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

fn main() {
    let log_file_path = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "/logs/rust-app.log".to_string());

    // Define possible log levels and messages
    let log_levels = ["INFO", "WARNING", "ERROR", "DEBUG"];
    let log_messages = [
        "User logged in successfully",
        "Disk space running low",
        "Failed to connect to database",
        "Configuration file missing",
        "Service restarted successfully",
        "Permission denied for user operation",
        "Background job completed",
        "Unable to fetch data from API",
    ];

    loop {
        // Open the log file in append mode
        let mut log_file = match OpenOptions::new().create(true).append(true).open(&log_file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                continue;
            }
        };

        // Randomly select a log level and message
        let log_level = log_levels.choose(&mut thread_rng()).unwrap();
        let log_message = log_messages.choose(&mut thread_rng()).unwrap();

        // Create the log entry
        let log_entry = format!(
            r#"{{"@timestamp": "{}", "level": "{}", "message": "{}"}}"#,
            Utc::now(),
            log_level,
            log_message
        );

        // Write the log entry to the file
        if let Err(e) = writeln!(log_file, "{}", log_entry) {
            eprintln!("Failed to write to log file: {}", e);
        } else {
            println!("Wrote log message to file: {}", log_entry);
        }

        // Random sleep interval between log entries
        let interval: u64 = env::var("LOG_INTERVAL")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(5);
        let randomized_interval = interval + thread_rng().gen_range(0..2);
        thread::sleep(Duration::from_secs(randomized_interval));
    }
}
