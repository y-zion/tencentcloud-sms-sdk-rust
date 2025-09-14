//! Main client for TencentCloud API requests

use crate::core::{ClientProfile, Credential};
use crate::error::{Result, TencentCloudError};
use crate::sms::{SendSmsRequest, SendSmsResponse};
use chrono::Utc;
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::time::Duration;
use tencentcloud_sign_sdk::{Tc3Signer, sha256_hex};

/// Main client for TencentCloud SMS API
pub struct Client {
    /// Credentials for authentication
    credential: Credential,
    /// Region for API requests
    region: String,
    /// Client configuration profile
    profile: ClientProfile,
    /// HTTP client
    http_client: reqwest::Client,
    /// Service name (always "sms" for SMS service)
    service: String,
    /// TC3 signer for request signing
    signer: Tc3Signer,
}

impl Client {
    /// Create a new client with credentials and region
    ///
    /// # Arguments
    ///
    /// * `credential` - TencentCloud credentials
    /// * `region` - Region for API requests (e.g., "ap-guangzhou")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tencentcloud_sms_sdk::{Client, Credential};
    ///
    /// let credential = Credential::new("your_secret_id", "your_secret_key", None);
    /// let client = Client::new(credential, "ap-guangzhou");
    /// ```
    pub fn new<S: Into<String>>(credential: Credential, region: S) -> Self {
        Self::with_profile(credential, region, ClientProfile::new())
    }

    /// Create a new client with custom profile
    ///
    /// # Arguments
    ///
    /// * `credential` - TencentCloud credentials
    /// * `region` - Region for API requests
    /// * `profile` - Client configuration profile
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tencentcloud_sms_sdk::{Client, Credential, ClientProfile, HttpProfile};
    ///
    /// let credential = Credential::new("your_secret_id", "your_secret_key", None);
    /// let mut http_profile = HttpProfile::new();
    /// http_profile.set_req_timeout(30);
    /// let client_profile = ClientProfile::with_http_profile(http_profile);
    /// let client = Client::with_profile(credential, "ap-guangzhou", client_profile);
    /// ```
    pub fn with_profile<S: Into<String>>(
        credential: Credential,
        region: S,
        profile: ClientProfile,
    ) -> Self {
        let http_profile = profile.get_http_profile();

        let mut client_builder = reqwest::Client::builder()
            .timeout(http_profile.get_req_timeout())
            .connect_timeout(http_profile.get_connect_timeout())
            .tcp_keepalive(if http_profile.keep_alive {
                Some(Duration::from_secs(60))
            } else {
                None
            })
            .user_agent(&http_profile.user_agent);

        // Configure proxy if set
        if let Some(proxy_url) = http_profile.get_proxy_url() {
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                client_builder = client_builder.proxy(proxy);
            }
        }

        let http_client = client_builder
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let signer = Tc3Signer::new(
            credential.secret_id().to_string(),
            credential.secret_key().to_string(),
            "sms".to_string(),
            profile.is_debug(),
        );

        Self {
            credential,
            region: region.into(),
            profile,
            http_client,
            service: "sms".to_string(),
            signer,
        }
    }

    /// Send SMS message
    ///
    /// # Arguments
    ///
    /// * `request` - SendSmsRequest containing SMS parameters
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tencentcloud_sms_sdk::{Client, Credential, SendSmsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let credential = Credential::new("your_secret_id", "your_secret_key", None);
    ///     let client = Client::new(credential, "ap-guangzhou");
    ///     
    ///     let request = SendSmsRequest::new(
    ///         vec!["+8613800000000".to_string()],
    ///         "1400000000",
    ///         "123456",
    ///         "YourSignature",
    ///         vec!["123456".to_string()],
    ///     );
    ///     
    ///     let response = client.send_sms(request).await?;
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_sms(&self, request: SendSmsRequest) -> Result<SendSmsResponse> {
        self.make_request("SendSms", &request).await
    }

    /// Make an API request
    async fn make_request<T, R>(&self, action: &str, request: &T) -> Result<R>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        // Validate credentials
        self.credential.validate()?;

        // Serialize request body
        let payload = serde_json::to_string(request)?;

        // Current timestamp
        let timestamp = Utc::now();

        // Build headers
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert(
            "Host".to_string(),
            self.profile.get_http_profile().endpoint.clone(),
        );
        headers.insert("X-TC-Action".to_string(), action.to_string());
        headers.insert(
            "X-TC-Version".to_string(),
            self.profile.get_api_version().to_string(),
        );
        headers.insert("X-TC-Region".to_string(), self.region.clone());
        headers.insert(
            "X-TC-Timestamp".to_string(),
            timestamp.timestamp().to_string(),
        );
        headers.insert(
            "X-TC-Language".to_string(),
            self.profile.get_language().to_string(),
        );

        // Add session token if available
        if let Some(token) = self.credential.token() {
            headers.insert("X-TC-Token".to_string(), token.to_string());
        }

        // Prepare headers for signing
        let host = self.profile.get_http_profile().endpoint.clone();
        let canonical_headers = format!(
            "content-type:application/json\nhost:{}\n",
            host
        );
        let signed_headers = "content-type;host";
        let hashed_payload = sha256_hex(&payload);

        // Sign the request using TC3 signer
        let result = self.signer.sign(
            &self.profile.get_http_profile().req_method,
            "/",
            "",
            &canonical_headers,
            signed_headers,
            &hashed_payload,
            timestamp.timestamp(),
        );

        // Create authorization header
        let authorization = self.signer.create_authorization_header(&result, signed_headers);
        headers.insert("Authorization".to_string(), authorization);

        // Build HTTP request
        let url = self.profile.get_http_profile().get_full_endpoint();
        let mut request_builder = match self.profile.get_http_profile().req_method.as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            _ => self.http_client.post(&url),
        };

        // Add headers
        for (key, value) in headers {
            request_builder = request_builder.header(&key, &value);
        }

        // Add body for POST requests
        if self.profile.get_http_profile().req_method == "POST" {
            request_builder = request_builder.body(payload.clone());
        }

        // Send request
        let response = request_builder.send().await?;

        // Check status code
        if !response.status().is_success() {
            return Err(TencentCloudError::other(format!(
                "HTTP error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        // Get response text
        let response_text = response.text().await?;

        // Debug logging
        if self.profile.is_debug() {
            log::debug!("Request: {}", payload);
            log::debug!("Response: {}", response_text);
        }

        // Parse response
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        // Check for API errors
        if let Some(error) = response_json.get("Response").and_then(|r| r.get("Error")) {
            let code = error
                .get("Code")
                .and_then(|c| c.as_str())
                .unwrap_or("Unknown");
            let message = error
                .get("Message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            let request_id = response_json
                .get("Response")
                .and_then(|r| r.get("RequestId"))
                .and_then(|r| r.as_str())
                .map(|s| s.to_string());

            return Err(TencentCloudError::api_with_request_id(
                code,
                message,
                request_id.as_deref(),
            ));
        }

        // Extract the actual response data
        let response_data = response_json
            .get("Response")
            .ok_or_else(|| TencentCloudError::other("Invalid response format"))?;

        // Deserialize response
        let result: R = serde_json::from_value(response_data.clone())?;

        Ok(result)
    }

    /// Get the region
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Get the service name
    pub fn service(&self) -> &str {
        &self.service
    }

    /// Get the client profile
    pub fn profile(&self) -> &ClientProfile {
        &self.profile
    }

    /// Set a new region
    pub fn set_region<S: Into<String>>(&mut self, region: S) {
        self.region = region.into();
    }

    /// Update the client profile
    pub fn set_profile(&mut self, profile: ClientProfile) {
        self.profile = profile.clone();
        // Update signer with new debug setting
        self.signer = Tc3Signer::new(
            self.credential.secret_id().to_string(),
            self.credential.secret_key().to_string(),
            "sms".to_string(),
            profile.is_debug(),
        );
    }

    /// Update credentials
    pub fn set_credential(&mut self, credential: Credential) {
        self.credential = credential.clone();
        self.signer = Tc3Signer::new(
            credential.secret_id().to_string(),
            credential.secret_key().to_string(),
            "sms".to_string(),
            self.profile.is_debug(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::HttpProfile;
    use crate::sms::SendSmsRequest;

    #[test]
    fn test_client_creation() {
        let credential = Credential::new("test_id", "test_key", None);
        let client = Client::new(credential, "ap-guangzhou");

        assert_eq!(client.region(), "ap-guangzhou");
        assert_eq!(client.service(), "sms");
    }

    #[test]
    fn test_client_with_profile() {
        let credential = Credential::new("test_id", "test_key", None);
        let mut http_profile = HttpProfile::new();
        http_profile.set_req_timeout(30);
        let client_profile = ClientProfile::with_http_profile(http_profile);
        let client = Client::with_profile(credential, "ap-guangzhou", client_profile);

        assert_eq!(client.region(), "ap-guangzhou");
        assert_eq!(client.profile().get_http_profile().req_timeout, 30);
    }

    #[test]
    fn test_client_setters() {
        let credential = Credential::new("test_id", "test_key", None);
        let mut client = Client::new(credential, "ap-guangzhou");

        client.set_region("ap-beijing");
        assert_eq!(client.region(), "ap-beijing");

        let new_credential = Credential::new("new_id", "new_key", None);
        client.set_credential(new_credential);
        assert_eq!(client.credential.secret_id(), "new_id");
    }

    #[tokio::test]
    async fn test_send_sms_invalid_credentials() {
        let credential = Credential::new("", "", None);
        let client = Client::new(credential, "ap-guangzhou");

        let request = SendSmsRequest::new(
            vec!["+8613800000000".to_string()],
            "1400000000",
            "123456",
            "Test",
            vec!["123456".to_string()],
        );

        let result = client.send_sms(request).await;
        assert!(result.is_err());
    }
}
