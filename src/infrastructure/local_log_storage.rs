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
        let remaining_logs: Vec<LogMessage> = logs.into_iter()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn local_log_storage_creates_log_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        LocalLogStorage::new(file_path.to_str().unwrap());
        assert!(file_path.exists());
    }

    #[test]
    fn local_log_storage_saves_log() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        let storage = LocalLogStorage::new(file_path.to_str().unwrap());

        let log = LogMessage { message: "Test log".to_string() };
        storage.save_log(&log);

        let contents = fs::read_to_string(file_path).unwrap();
        assert!(contents.contains("\"message\":\"Test log\""));
    }

    #[test]
    fn local_log_storage_fetches_unsent_logs() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        let storage = LocalLogStorage::new(file_path.to_str().unwrap());

        let log = LogMessage { message: "Test log".to_string() };
        storage.save_log(&log);

        let logs = storage.fetch_unsent_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Test log");
    }

    /* #[test]
    fn local_log_storage_clears_sent_logs() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        let storage = LocalLogStorage::new(file_path.to_str().unwrap());

        let log1 = LogMessage { message: "Log 1".to_string() };
        let log2 = LogMessage { message: "Log 2".to_string() };
        storage.save_log(&log1);
        storage.save_log(&log2);

        storage.clear_sent_logs(&[log1]);

        let logs = storage.fetch_unsent_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Log 2");
    }
*/

    #[test]
    fn local_log_storage_handles_empty_log_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        let storage = LocalLogStorage::new(file_path.to_str().unwrap());

        let logs = storage.fetch_unsent_logs();
        assert!(logs.is_empty());
    }

    #[test]
    fn local_log_storage_handles_invalid_log_entries() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("log.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "invalid log entry").unwrap();

        let storage = LocalLogStorage::new(file_path.to_str().unwrap());
        let logs = storage.fetch_unsent_logs();
        assert!(logs.is_empty());
    }
}