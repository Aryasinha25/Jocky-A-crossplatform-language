use serde::Serialize;
use std::env;
use std::time::Duration;

#[derive(Serialize)]
struct IngestRequest<'a, T> {
    endpoint_id: &'a str,
    investigation: &'a T,
}

pub struct ControlPlaneClient {
    api_url: String,
    api_key: String,
    endpoint_id: String,
    client: reqwest::Client,
}

impl ControlPlaneClient {
    pub fn from_env() -> Result<Self, String> {
        let api_url = env::var("JOCKY_API_URL")
            .map_err(|_| "JOCKY_API_URL environment variable is not set")?;
        let api_key = env::var("JOCKY_API_KEY")
            .map_err(|_| "JOCKY_API_KEY environment variable is not set")?;
        let endpoint_id = env::var("JOCKY_ENDPOINT_ID")
            .map_err(|_| "JOCKY_ENDPOINT_ID environment variable is not set")?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        Ok(Self {
            api_url,
            api_key,
            endpoint_id,
            client,
        })
    }

    pub async fn submit_investigation<T: Serialize>(&self, result: &T) -> Result<(), String> {
        let payload = IngestRequest {
            endpoint_id: &self.endpoint_id,
            investigation: result,
        };

        let url = format!(
            "{}/api/v1/investigations",
            self.api_url.trim_end_matches('/')
        );

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error submitting investigation: {}", e))?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "".to_string());

        if !status.is_success() {
            return Err(format!(
                "Server rejected submission with HTTP {} - {}",
                status.as_u16(),
                body
            ));
        }

        Ok(())
    }
}
