use crate::domain::log::LogMessage;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use tracing::{error, info};

pub struct LocalLogStorage {
    file_path: String,
    lock: Mutex<()>,
}

impl LocalLogStorage {
    pub fn new(file_path: &str) -> Self {
        // Ensure the log file exists
        if let Err(e) = OpenOptions::new().create(true).append(true).open(file_path) {
            error!("Failed to initialize log file: {:?}", e);
        }

        Self {
            file_path: file_path.to_string(),
            lock: Mutex::new(()),
        }
    }

    /// Save logs to a file if Kafka is down
    pub fn save_log(&self, log: &LogMessage) {
        let _guard = self.lock.lock().unwrap(); // Ensure thread safety
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .expect("Failed to open log file");

        let log_entry = serde_json::to_string(log).unwrap();
        if let Err(e) = writeln!(file, "{}", log_entry) {
            error!("❌ Failed to write log to file: {:?}", e);
        } else {
            info!("📂 Log saved locally: {:?}", log);
        }
    }

    /// Read all unsent logs from the file
    pub fn fetch_unsent_logs(&self) -> Vec<LogMessage> {
        let _guard = self.lock.lock().unwrap();
        let file = File::open(&self.file_path);

        if file.is_err() {
            return vec![];
        }

        let reader = BufReader::new(file.unwrap());
        let mut logs = Vec::new();

        for line in reader.lines() {
            if let Ok(json_line) = line {
                if let Ok(log) = serde_json::from_str::<LogMessage>(&json_line) {
                    logs.push(log);
                }
            }
        }

        logs
    }

    /// Remove successfully sent logs by truncating the file
    pub fn clear_sent_logs(&self, successfully_sent: &[LogMessage]) {
        let _guard = self.lock.lock().unwrap();

        let logs = self.fetch_unsent_logs();
        let remaining_logs: Vec<LogMessage> = logs
            .into_iter()
            .filter(|log| !successfully_sent.contains(log))
            .collect();

        // Rewrite file with only unsent logs
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to truncate log file");

        for log in remaining_logs {
            if let Err(e) = writeln!(file, "{}", serde_json::to_string(&log).unwrap()) {
                error!("❌ Failed to rewrite log file: {:?}", e);
            }
        }
    }
}
