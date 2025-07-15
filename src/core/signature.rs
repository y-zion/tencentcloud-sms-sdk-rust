//! TC3-HMAC-SHA256 signature implementation for TencentCloud API authentication

use crate::error::{Result, TencentCloudError};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// TC3-HMAC-SHA256 signature generator
pub struct Signer {
    secret_id: String,
    secret_key: String,
    token: Option<String>,
}

impl Signer {
    /// Create a new signer with credentials
    pub fn new<S: Into<String>>(
        secret_id: S,
        secret_key: S,
        token: Option<S>,
    ) -> Self {
        Self {
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            token: token.map(|t| t.into()),
        }
    }

    /// Sign a request using TC3-HMAC-SHA256 algorithm
    pub fn sign_request(
        &self,
        http_method: &str,
        uri: &str,
        query_string: &str,
        headers: &HashMap<String, String>,
        payload: &str,
        service: &str,
        region: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<String> {
        // Step 1: Create canonical request
        let canonical_request = self.create_canonical_request(
            http_method,
            uri,
            query_string,
            headers,
            payload,
        )?;

        // Step 2: Create string to sign
        let string_to_sign = self.create_string_to_sign(
            &canonical_request,
            service,
            region,
            timestamp,
        )?;

        // Step 3: Calculate signature
        let signature = self.calculate_signature(
            &string_to_sign,
            service,
            region,
            timestamp,
        )?;

        Ok(signature)
    }

    /// Create canonical request string
    fn create_canonical_request(
        &self,
        http_method: &str,
        uri: &str,
        query_string: &str,
        headers: &HashMap<String, String>,
        payload: &str,
    ) -> Result<String> {
        // Canonical URI
        let canonical_uri = if uri.is_empty() { "/" } else { uri };

        // Canonical query string
        let canonical_query_string = self.create_canonical_query_string(query_string)?;

        // Canonical headers
        let (canonical_headers, signed_headers) = self.create_canonical_headers(headers)?;

        // Hashed payload
        let hashed_payload = self.hash_payload(payload);

        // Create canonical request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            http_method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            hashed_payload
        );

        Ok(canonical_request)
    }

    /// Create canonical query string
    fn create_canonical_query_string(&self, query_string: &str) -> Result<String> {
        if query_string.is_empty() {
            return Ok(String::new());
        }

        let mut params: Vec<(String, String)> = Vec::new();
        
        for param in query_string.split('&') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                params.push((
                    Self::url_encode(parts[0]),
                    Self::url_encode(parts[1]),
                ));
            } else if parts.len() == 1 {
                params.push((Self::url_encode(parts[0]), String::new()));
            }
        }

        // Sort parameters by key
        params.sort_by(|a, b| a.0.cmp(&b.0));

        let canonical_query = params
            .into_iter()
            .map(|(k, v)| if v.is_empty() { k } else { format!("{}={}", k, v) })
            .collect::<Vec<String>>()
            .join("&");

        Ok(canonical_query)
    }

    /// Create canonical headers and signed headers
    fn create_canonical_headers(
        &self,
        headers: &HashMap<String, String>,
    ) -> Result<(String, String)> {
        let mut canonical_headers = Vec::new();
        let mut header_names = Vec::new();

        for (key, value) in headers {
            let lower_key = key.to_lowercase();
            let trimmed_value = value.trim();
            
            canonical_headers.push(format!("{}:{}", lower_key, trimmed_value));
            header_names.push(lower_key);
        }

        // Sort headers by key
        canonical_headers.sort();
        header_names.sort();

        let canonical_headers_string = canonical_headers.join("\n") + "\n";
        let signed_headers_string = header_names.join(";");

        Ok((canonical_headers_string, signed_headers_string))
    }

    /// Hash payload using SHA256
    fn hash_payload(&self, payload: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Create string to sign
    fn create_string_to_sign(
        &self,
        canonical_request: &str,
        service: &str,
        _region: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<String> {
        let algorithm = "TC3-HMAC-SHA256";
        let date = timestamp.format("%Y%m%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, service);
        
        let mut hasher = Sha256::new();
        hasher.update(canonical_request.as_bytes());
        let hashed_canonical_request = hex::encode(hasher.finalize());

        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm,
            timestamp.timestamp(),
            credential_scope,
            hashed_canonical_request
        );

        Ok(string_to_sign)
    }

    /// Calculate signature
    fn calculate_signature(
        &self,
        string_to_sign: &str,
        service: &str,
        _region: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<String> {
        let date = timestamp.format("%Y%m%d").to_string();
        
        // Calculate signing key
        let mut mac = HmacSha256::new_from_slice(
            format!("TC3{}", self.secret_key).as_bytes()
        ).map_err(|e| TencentCloudError::signature(format!("Failed to create HMAC: {}", e)))?;
        
        mac.update(date.as_bytes());
        let k_date = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&k_date)
            .map_err(|e| TencentCloudError::signature(format!("Failed to create HMAC: {}", e)))?;
        mac.update(service.as_bytes());
        let k_service = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&k_service)
            .map_err(|e| TencentCloudError::signature(format!("Failed to create HMAC: {}", e)))?;
        mac.update(b"tc3_request");
        let k_signing = mac.finalize().into_bytes();

        // Calculate signature
        let mut mac = HmacSha256::new_from_slice(&k_signing)
            .map_err(|e| TencentCloudError::signature(format!("Failed to create HMAC: {}", e)))?;
        mac.update(string_to_sign.as_bytes());
        let signature = mac.finalize().into_bytes();

        Ok(hex::encode(signature))
    }

    /// Create authorization header
    pub fn create_authorization_header(
        &self,
        signature: &str,
        service: &str,
        _region: &str,
        timestamp: DateTime<Utc>,
        signed_headers: &str,
    ) -> String {
        let date = timestamp.format("%Y%m%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, service);
        let credential = format!("{}/{}", self.secret_id, credential_scope);

        format!(
            "TC3-HMAC-SHA256 Credential={}, SignedHeaders={}, Signature={}",
            credential, signed_headers, signature
        )
    }

    /// Get signed headers from headers map
    pub fn get_signed_headers(headers: &HashMap<String, String>) -> String {
        let mut header_names: Vec<String> = headers
            .keys()
            .map(|k| k.to_lowercase())
            .collect();
        header_names.sort();
        header_names.join(";")
    }

    /// URL encode string
    fn url_encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_hash_payload() {
        let signer = Signer::new("test_id", "test_key", None);
        let payload = r#"{"test": "value"}"#;
        let hash = signer.hash_payload(payload);
        
        // This should produce a consistent SHA256 hash
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex characters
    }

    #[test]
    fn test_canonical_query_string() {
        let signer = Signer::new("test_id", "test_key", None);
        
        // Test empty query string
        let result = signer.create_canonical_query_string("").unwrap();
        assert_eq!(result, "");
        
        // Test single parameter
        let result = signer.create_canonical_query_string("param1=value1").unwrap();
        assert_eq!(result, "param1=value1");
        
        // Test multiple parameters (should be sorted)
        let result = signer.create_canonical_query_string("param2=value2&param1=value1").unwrap();
        assert_eq!(result, "param1=value1&param2=value2");
    }

    #[test]
    fn test_canonical_headers() {
        let signer = Signer::new("test_id", "test_key", None);
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Host".to_string(), "sms.tencentcloudapi.com".to_string());
        
        let (canonical_headers, signed_headers) = signer.create_canonical_headers(&headers).unwrap();
        
        // Headers should be sorted by key (lowercase)
        assert!(canonical_headers.contains("content-type:application/json"));
        assert!(canonical_headers.contains("host:sms.tencentcloudapi.com"));
        assert_eq!(signed_headers, "content-type;host");
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(Signer::url_encode("hello world"), "hello+world");
        assert_eq!(Signer::url_encode("hello@world"), "hello%40world");
        assert_eq!(Signer::url_encode("hello"), "hello");
    }

    #[test]
    fn test_create_authorization_header() {
        let signer = Signer::new("test_id", "test_key", None);
        let timestamp = Utc.timestamp_opt(1609459200, 0).unwrap(); // 2021-01-01 00:00:00 UTC
        
        let auth_header = signer.create_authorization_header(
            "test_signature",
            "sms",
            "ap-guangzhou",
            timestamp,
            "content-type;host"
        );
        
        assert!(auth_header.starts_with("TC3-HMAC-SHA256 Credential=test_id/20210101/sms/tc3_request"));
        assert!(auth_header.contains("SignedHeaders=content-type;host"));
        assert!(auth_header.contains("Signature=test_signature"));
    }
}