use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ErrorTracker {
    error_tracker: HashMap<String, u64>,
    max_age: Duration,
}

impl ErrorTracker {
    pub fn new(max_age: Duration) -> Self {
        Self {
            error_tracker: HashMap::new(),
            max_age,
        }
    }

    pub fn duplicate_check(&mut self, message: &str) -> bool {
        let current_timestamp = Self::current_timestamp();
        if let Some(&timestamp) = self.error_tracker.get(message) {
            if current_timestamp - timestamp <= self.max_age.as_secs() {
                return true;
            }
        }
        false
    }

    pub fn add_timestamp(&mut self, message: &str) {
        let current_timestamp = Self::current_timestamp();
        self.error_tracker.insert(message.to_string(), current_timestamp);
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}
