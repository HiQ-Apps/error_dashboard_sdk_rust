use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use crate::error_tracker::ErrorTracker;
use crate::configuration::Configuration;
use crate::fetch::{error_dashboard_fetch, CustomFetchProps, ErrorPayload, ErrorResponseType};

#[derive(Debug)]
pub struct ErrorDashboardClient {
    client_id: String,
    client_secret: String,
    client: Client,
    error_tracker: Arc<Mutex<ErrorTracker>>,
    configs: Arc<Mutex<Configuration>>,
}

impl ErrorDashboardClient {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        let error_tracker = Arc::new(Mutex::new(ErrorTracker::new(Duration::from_secs(3600))));
        let configs = Arc::new(Mutex::new(Configuration::default()));
        let client = Client::new();

        ErrorDashboardClient {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            client,
            error_tracker,
            configs,
        }
    }

    pub fn initialize(client_id: &str, client_secret: &str) -> Arc<Self> {
        Arc::new(Self::new(client_id, client_secret))
    }

    pub async fn send_error<E>(&self, error: E, message: &str)
    where
        E: Into<Box<dyn Error + Send + Sync>> + 'static,
    {
        let boxed_error: Box<dyn Error + Send + Sync> = error.into();
        let error_details = format!("{}", boxed_error);

        let configs = self.configs.lock().unwrap();

        let retry_attempts = match configs.get_config(ConfigKey::RetryAttempts) {
            ConfigValue::Usize(val) => val,
            _ => 3,
        };

        let retry_delay = match configs.get_config(ConfigKey::RetryDelay) {
            ConfigValue::U64(val) => val,
            _ => 3000,
        };

        let verbose = match configs.get_config(ConfigKey::Verbose) {
            ConfigValue::Bool(val) => val,
            _ => false,
        };


        let payload = ErrorPayload {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            message,
            error_details: &error_details,
        };

        let props = CustomFetchProps {
            client_secret: &self.client_secret,
            client_id: &self.client_id,
            headers: None,
            endpoint: "https://higuard-error-dashboard.shuttleapp.rs/sdk/error",
            body: Some(payload),
            retry_attempts,
            retry_delay: Duration::from_millis(retry_delay as u64),
        };

        let current_time = chrono::Utc::now().timestamp_millis() as u64;

        if self.error_tracker.lock().unwrap().duplicate_check(message) {
            if verbose {
                println!("Duplicate error detected, not sending");
            }
            return;
        }

        match error_dashboard_fetch(&self.client, props).await {
            Ok(ErrorResponseType { is_success, .. }) if is_success => {
                if verbose {
                    println!("Error sent successfully");
                }
                self.error_tracker.lock().unwrap().add_timestamp(message);
            }
            Ok(ErrorResponseType { is_error, .. }) if is_error => {
                if verbose {
                    println!("Error sending data to Higuard");
                }
            }
            Err(e) => {
                if verbose {
                    println!("Error while sending the error: {}", e);
                }
            }
        }
    }

    pub fn override_configs(&self, new_configs: Configuration) {
        let mut configs = self.configs.lock().unwrap();
        *configs = new_configs;
    }

    pub async fn static_send_error<E>(client: Arc<ErrorDashboardClient>, error: E, message: &str)
    where
        E: Into<Box<dyn Error + Send + Sync>> + 'static,
    {
        client.send_error(error, message).await;
    }
}

