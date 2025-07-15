//! Configuration profiles for HTTP and client settings

use std::time::Duration;

/// HTTP configuration profile
#[derive(Debug, Clone)]
pub struct HttpProfile {
    /// HTTP request method (GET, POST)
    pub req_method: String,
    /// API endpoint URL
    pub endpoint: String,
    /// Request timeout in seconds
    pub req_timeout: u64,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Keep-alive setting
    pub keep_alive: bool,
    /// Proxy host (optional)
    pub proxy_host: Option<String>,
    /// Proxy port (optional)
    pub proxy_port: Option<u16>,
    /// User-Agent header
    pub user_agent: String,
}

impl HttpProfile {
    /// Create a new HTTP profile with default settings
    pub fn new() -> Self {
        Self {
            req_method: "POST".to_string(),
            endpoint: "sms.tencentcloudapi.com".to_string(),
            req_timeout: 60,
            connect_timeout: 60,
            keep_alive: false,
            proxy_host: None,
            proxy_port: None,
            user_agent: "TencentCloud-SDK-Rust/1.0.0".to_string(),
        }
    }

    /// Set the HTTP request method
    pub fn set_req_method<S: Into<String>>(&mut self, method: S) -> &mut Self {
        self.req_method = method.into();
        self
    }

    /// Set the API endpoint
    pub fn set_endpoint<S: Into<String>>(&mut self, endpoint: S) -> &mut Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Set the request timeout in seconds
    pub fn set_req_timeout(&mut self, timeout: u64) -> &mut Self {
        self.req_timeout = timeout;
        self
    }

    /// Set the connection timeout in seconds
    pub fn set_connect_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set the keep-alive setting
    pub fn set_keep_alive(&mut self, keep_alive: bool) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }

    /// Set the proxy host
    pub fn set_proxy_host<S: Into<String>>(&mut self, host: Option<S>) -> &mut Self {
        self.proxy_host = host.map(|h| h.into());
        self
    }

    /// Set the proxy port
    pub fn set_proxy_port(&mut self, port: Option<u16>) -> &mut Self {
        self.proxy_port = port;
        self
    }

    /// Set the User-Agent header
    pub fn set_user_agent<S: Into<String>>(&mut self, user_agent: S) -> &mut Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Get the full endpoint URL with protocol
    pub fn get_full_endpoint(&self) -> String {
        if self.endpoint.starts_with("http://") || self.endpoint.starts_with("https://") {
            self.endpoint.clone()
        } else {
            format!("https://{}", self.endpoint)
        }
    }

    /// Get request timeout as Duration
    pub fn get_req_timeout(&self) -> Duration {
        Duration::from_secs(self.req_timeout)
    }

    /// Get connection timeout as Duration
    pub fn get_connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout)
    }

    /// Check if proxy is configured
    pub fn has_proxy(&self) -> bool {
        self.proxy_host.is_some() && self.proxy_port.is_some()
    }

    /// Get proxy URL if configured
    pub fn get_proxy_url(&self) -> Option<String> {
        if let (Some(host), Some(port)) = (&self.proxy_host, self.proxy_port) {
            Some(format!("http://{}:{}", host, port))
        } else {
            None
        }
    }
}

impl Default for HttpProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Client configuration profile
#[derive(Debug, Clone)]
pub struct ClientProfile {
    /// HTTP profile for request settings
    pub http_profile: HttpProfile,
    /// Signature method (default: HmacSHA256)
    pub sign_method: String,
    /// API version
    pub api_version: String,
    /// Language for error messages
    pub language: String,
    /// Debug mode
    pub debug: bool,
}

impl ClientProfile {
    /// Create a new client profile with default settings
    pub fn new() -> Self {
        Self {
            http_profile: HttpProfile::new(),
            sign_method: "HmacSHA256".to_string(),
            api_version: "2021-01-11".to_string(),
            language: "en-US".to_string(),
            debug: false,
        }
    }

    /// Create a new client profile with custom HTTP profile
    pub fn with_http_profile(http_profile: HttpProfile) -> Self {
        Self {
            http_profile,
            sign_method: "HmacSHA256".to_string(),
            api_version: "2021-01-11".to_string(),
            language: "en-US".to_string(),
            debug: false,
        }
    }

    /// Set the HTTP profile
    pub fn set_http_profile(&mut self, profile: HttpProfile) -> &mut Self {
        self.http_profile = profile;
        self
    }

    /// Set the signature method
    pub fn set_sign_method<S: Into<String>>(&mut self, method: S) -> &mut Self {
        self.sign_method = method.into();
        self
    }

    /// Set the API version
    pub fn set_api_version<S: Into<String>>(&mut self, version: S) -> &mut Self {
        self.api_version = version.into();
        self
    }

    /// Set the language
    pub fn set_language<S: Into<String>>(&mut self, language: S) -> &mut Self {
        self.language = language.into();
        self
    }

    /// Set the debug mode
    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    /// Get the HTTP profile
    pub fn get_http_profile(&self) -> &HttpProfile {
        &self.http_profile
    }

    /// Get the signature method
    pub fn get_sign_method(&self) -> &str {
        &self.sign_method
    }

    /// Get the API version
    pub fn get_api_version(&self) -> &str {
        &self.api_version
    }

    /// Get the language
    pub fn get_language(&self) -> &str {
        &self.language
    }

    /// Check if debug mode is enabled
    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Default for ClientProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_profile_defaults() {
        let profile = HttpProfile::new();
        assert_eq!(profile.req_method, "POST");
        assert_eq!(profile.endpoint, "sms.tencentcloudapi.com");
        assert_eq!(profile.req_timeout, 60);
        assert_eq!(profile.connect_timeout, 60);
        assert!(!profile.keep_alive);
        assert!(profile.proxy_host.is_none());
        assert!(profile.proxy_port.is_none());
    }

    #[test]
    fn test_http_profile_configuration() {
        let mut profile = HttpProfile::new();
        profile
            .set_req_method("GET")
            .set_endpoint("custom.endpoint.com")
            .set_req_timeout(30)
            .set_connect_timeout(30)
            .set_keep_alive(true)
            .set_proxy_host(Some("proxy.example.com"))
            .set_proxy_port(Some(8080));

        assert_eq!(profile.req_method, "GET");
        assert_eq!(profile.endpoint, "custom.endpoint.com");
        assert_eq!(profile.req_timeout, 30);
        assert_eq!(profile.connect_timeout, 30);
        assert!(profile.keep_alive);
        assert_eq!(profile.proxy_host, Some("proxy.example.com".to_string()));
        assert_eq!(profile.proxy_port, Some(8080));
    }

    #[test]
    fn test_http_profile_full_endpoint() {
        let mut profile = HttpProfile::new();
        assert_eq!(profile.get_full_endpoint(), "https://sms.tencentcloudapi.com");

        profile.set_endpoint("http://custom.endpoint.com");
        assert_eq!(profile.get_full_endpoint(), "http://custom.endpoint.com");

        profile.set_endpoint("https://custom.endpoint.com");
        assert_eq!(profile.get_full_endpoint(), "https://custom.endpoint.com");
    }

    #[test]
    fn test_http_profile_proxy() {
        let mut profile = HttpProfile::new();
        assert!(!profile.has_proxy());
        assert!(profile.get_proxy_url().is_none());

        profile.set_proxy_host(Some("proxy.example.com"));
        assert!(!profile.has_proxy()); // Still false because port is not set

        profile.set_proxy_port(Some(8080));
        assert!(profile.has_proxy());
        assert_eq!(profile.get_proxy_url(), Some("http://proxy.example.com:8080".to_string()));
    }

    #[test]
    fn test_client_profile_defaults() {
        let profile = ClientProfile::new();
        assert_eq!(profile.sign_method, "HmacSHA256");
        assert_eq!(profile.api_version, "2021-01-11");
        assert_eq!(profile.language, "en-US");
        assert!(!profile.debug);
    }

    #[test]
    fn test_client_profile_configuration() {
        let mut profile = ClientProfile::new();
        profile
            .set_sign_method("HmacSHA1")
            .set_api_version("2019-07-11")
            .set_language("zh-CN")
            .set_debug(true);

        assert_eq!(profile.sign_method, "HmacSHA1");
        assert_eq!(profile.api_version, "2019-07-11");
        assert_eq!(profile.language, "zh-CN");
        assert!(profile.debug);
    }
}