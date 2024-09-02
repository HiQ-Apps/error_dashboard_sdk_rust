use reqwest::{Client, header::HeaderMap};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct ErrorResponseType {
    pub is_success: bool,
    pub is_error: bool,
}

#[derive(Serialize)]
pub struct ErrorPayload<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub message: &'a str,
    pub error_details: &'a str,
}

pub struct CustomFetchProps<'a> {
    pub client_secret: &'a str,
    pub client_id: &'a str,
    pub headers: Option<HeaderMap>,
    pub endpoint: &'a str,
    pub body: Option<ErrorPayload<'a>>,
    pub retry_attempts: usize,
    pub retry_delay: Duration,
}

pub async fn error_dashboard_fetch(
    client: &Client,
    props: CustomFetchProps<'_>,
) -> Result<ErrorResponseType, reqwest::Error> {
    let mut is_success = false;
    let mut is_error = false;

    let mut headers = HeaderMap::new();
    headers.insert("client_secret", props.client_secret.parse().unwrap());
    headers.insert("client_id", props.client_id.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    if let Some(custom_headers) = props.headers {
        for (key, value) in custom_headers {
            headers.insert(key, value);
        }
    }

    let request_builder = client.post(props.endpoint).headers(headers);

    let request_builder = if let Some(body) = props.body {
        request_builder.json(&body)
    } else {
        request_builder
    };

    for attempt in 0..props.retry_attempts {
        match request_builder.try_clone().unwrap().send().await {
            Ok(response) => {
                if response.status().is_success() {
                    is_success = true;
                    return Ok(ErrorResponseType { is_success, is_error });
                } else {
                    is_error = true;
                }
            }
            Err(e) => {
                println!("Fetch error: {}", e);
                is_error = true;
            }
        }

        if attempt < props.retry_attempts - 1 {
            sleep(props.retry_delay).await;
        }
    }

    Ok(ErrorResponseType { is_success, is_error })
}
