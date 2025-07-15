//! Error types for the TencentCloud SMS SDK

use thiserror::Error;

/// Result type alias for TencentCloud operations
pub type Result<T> = std::result::Result<T, TencentCloudError>;

/// Main error type for TencentCloud SDK operations
#[derive(Error, Debug)]
pub enum TencentCloudError {
    /// Network-related errors
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API errors returned by TencentCloud
    #[error("API error: {code} - {message}")]
    Api {
        /// Error code returned by the API
        code: String,
        /// Error message returned by the API
        message: String,
        /// Request ID for debugging
        request_id: Option<String>,
    },

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Parameter validation errors
    #[error("Parameter error: {0}")]
    Parameter(String),

    /// Signature generation errors
    #[error("Signature error: {0}")]
    Signature(String),

    /// Timeout errors
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Generic errors
    #[error("Error: {0}")]
    Other(String),
}

impl TencentCloudError {
    /// Create a new API error
    pub fn api<S: Into<String>>(code: S, message: S) -> Self {
        Self::Api {
            code: code.into(),
            message: message.into(),
            request_id: None,
        }
    }

    /// Create a new API error with request ID
    pub fn api_with_request_id<S: Into<String>>(
        code: S,
        message: S,
        request_id: Option<S>,
    ) -> Self {
        Self::Api {
            code: code.into(),
            message: message.into(),
            request_id: request_id.map(|s| s.into()),
        }
    }

    /// Create a new authentication error
    pub fn auth<S: Into<String>>(message: S) -> Self {
        Self::Auth(message.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create a new parameter error
    pub fn parameter<S: Into<String>>(message: S) -> Self {
        Self::Parameter(message.into())
    }

    /// Create a new signature error
    pub fn signature<S: Into<String>>(message: S) -> Self {
        Self::Signature(message.into())
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(message: S) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new generic error
    pub fn other<S: Into<String>>(message: S) -> Self {
        Self::Other(message.into())
    }

    /// Get error code if this is an API error
    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Api { code, .. } => Some(code),
            _ => None,
        }
    }

    /// Get request ID if available
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Check if this is a specific API error code
    pub fn is_api_error(&self, error_code: &str) -> bool {
        match self {
            Self::Api { code, .. } => code == error_code,
            _ => false,
        }
    }

    /// Check if this is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::Network(_))
    }

    /// Check if this is a timeout error
    pub fn is_timeout_error(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    /// Print all error details (similar to C++ SDK)
    pub fn print_all(&self) -> String {
        match self {
            Self::Api {
                code,
                message,
                request_id,
            } => {
                if let Some(req_id) = request_id {
                    format!("API Error: {} - {} (Request ID: {})", code, message, req_id)
                } else {
                    format!("API Error: {} - {}", code, message)
                }
            }
            _ => self.to_string(),
        }
    }
}

/// Common API error codes
pub mod error_codes {
    /// Failed operation - signature incorrect or unapproved
    pub const SIGNATURE_INCORRECT_OR_UNAPPROVED: &str =
        "FailedOperation.SignatureIncorrectOrUnapproved";

    /// Failed operation - template incorrect or unapproved
    pub const TEMPLATE_INCORRECT_OR_UNAPPROVED: &str =
        "FailedOperation.TemplateIncorrectOrUnapproved";

    /// Unauthorized operation - SMS SDK app ID verify fail
    pub const SMS_SDK_APP_ID_VERIFY_FAIL: &str = "UnauthorizedOperation.SmsSdkAppIdVerifyFail";

    /// Invalid parameter - incorrect phone number
    pub const INCORRECT_PHONE_NUMBER: &str = "InvalidParameterValue.IncorrectPhoneNumber";

    /// Limit exceeded - phone number count limit
    pub const PHONE_NUMBER_COUNT_LIMIT: &str = "LimitExceeded.PhoneNumberCountLimit";

    /// Failed operation - insufficient balance in SMS package
    pub const INSUFFICIENT_BALANCE: &str = "FailedOperation.InsufficientBalanceInSmsPackage";

    /// Internal error - timeout
    pub const TIMEOUT: &str = "InternalError.Timeout";

    /// Request time exception
    pub const REQUEST_TIME_EXCEPTION: &str = "InternalError.RequestTimeException";
}
