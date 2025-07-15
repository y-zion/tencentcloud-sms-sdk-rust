//! Credential management for TencentCloud authentication

use crate::error::{Result, TencentCloudError};
use std::env;

/// TencentCloud credentials for API authentication
#[derive(Debug, Clone)]
pub struct Credential {
    /// Secret ID for authentication
    pub secret_id: String,
    /// Secret Key for authentication
    pub secret_key: String,
    /// Session token for temporary credentials (optional)
    pub token: Option<String>,
}

impl Credential {
    /// Create a new Credential instance
    ///
    /// # Arguments
    ///
    /// * `secret_id` - The secret ID for authentication
    /// * `secret_key` - The secret key for authentication
    /// * `token` - Optional session token for temporary credentials
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tencentcloud_sms_sdk::Credential;
    ///
    /// let credential = Credential::new("your_secret_id", "your_secret_key", None);
    /// ```
    pub fn new<S: Into<String>>(secret_id: S, secret_key: S, token: Option<S>) -> Self {
        Self {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            token: token.map(|t| t.into()),
        }
    }

    /// Create credentials from environment variables
    ///
    /// Reads the following environment variables:
    /// - `TENCENTCLOUD_SECRET_ID` or `TC_SECRET_ID`
    /// - `TENCENTCLOUD_SECRET_KEY` or `TC_SECRET_KEY`
    /// - `TENCENTCLOUD_TOKEN` or `TC_TOKEN` (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tencentcloud_sms_sdk::Credential;
    ///
    /// // Make sure to set environment variables first
    /// // export TENCENTCLOUD_SECRET_ID=your_secret_id
    /// // export TENCENTCLOUD_SECRET_KEY=your_secret_key
    /// let credential = Credential::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self> {
        let secret_id = env::var("TENCENTCLOUD_SECRET_ID")
            .or_else(|_| env::var("TC_SECRET_ID"))
            .map_err(|_| {
                TencentCloudError::auth(
                    "TENCENTCLOUD_SECRET_ID or TC_SECRET_ID environment variable not found",
                )
            })?;

        let secret_key = env::var("TENCENTCLOUD_SECRET_KEY")
            .or_else(|_| env::var("TC_SECRET_KEY"))
            .map_err(|_| {
                TencentCloudError::auth(
                    "TENCENTCLOUD_SECRET_KEY or TC_SECRET_KEY environment variable not found",
                )
            })?;

        let token = env::var("TENCENTCLOUD_TOKEN")
            .or_else(|_| env::var("TC_TOKEN"))
            .ok();

        Ok(Self {
            secret_id,
            secret_key,
            token,
        })
    }

    /// Validate that the credential has required fields
    pub fn validate(&self) -> Result<()> {
        if self.secret_id.is_empty() {
            return Err(TencentCloudError::auth("Secret ID cannot be empty"));
        }
        if self.secret_key.is_empty() {
            return Err(TencentCloudError::auth("Secret Key cannot be empty"));
        }
        Ok(())
    }

    /// Get the secret ID
    pub fn secret_id(&self) -> &str {
        &self.secret_id
    }

    /// Get the secret key
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }

    /// Get the session token
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Check if this credential has a session token
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// Update the session token
    pub fn set_token<S: Into<String>>(&mut self, token: Option<S>) {
        self.token = token.map(|t| t.into());
    }
}

impl Default for Credential {
    fn default() -> Self {
        Self {
            secret_id: String::new(),
            secret_key: String::new(),
            token: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_new() {
        let credential = Credential::new("test_id", "test_key", Some("test_token"));
        assert_eq!(credential.secret_id, "test_id");
        assert_eq!(credential.secret_key, "test_key");
        assert_eq!(credential.token, Some("test_token".to_string()));
    }

    #[test]
    fn test_credential_validate() {
        let credential = Credential::new("test_id", "test_key", None);
        assert!(credential.validate().is_ok());

        let invalid_credential = Credential::new("", "test_key", None);
        assert!(invalid_credential.validate().is_err());

        let invalid_credential = Credential::new("test_id", "", None);
        assert!(invalid_credential.validate().is_err());
    }

    #[test]
    fn test_credential_methods() {
        let mut credential = Credential::new("test_id", "test_key", None);

        assert_eq!(credential.secret_id(), "test_id");
        assert_eq!(credential.secret_key(), "test_key");
        assert_eq!(credential.token(), None);
        assert!(!credential.has_token());

        credential.set_token(Some("new_token"));
        assert_eq!(credential.token(), Some("new_token"));
        assert!(credential.has_token());
    }
}
