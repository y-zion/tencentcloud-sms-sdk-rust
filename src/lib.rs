//! # TencentCloud SMS SDK for Rust
//!
//! This crate provides a Rust implementation of the TencentCloud SMS SDK,
//! allowing you to send SMS messages through TencentCloud's SMS service.
//!
//! ## Features
//!
//! - Send SMS messages (verification codes, notifications, marketing)
//! - Support for both domestic and international SMS
//! - Async/await support with tokio
//! - TC3-HMAC-SHA256 signature algorithm
//! - Comprehensive error handling
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use tencentcloud_sms_sdk::{Client, Credential, SendSmsRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create credentials
//!     let credential = Credential::new("your_secret_id", "your_secret_key", None);
//!     
//!     // Create client
//!     let client = Client::new(credential, "ap-guangzhou");
//!     
//!     // Create request
//!     let request = SendSmsRequest::new(
//!         vec!["+8613800000000".to_string()],
//!         "1400000000",
//!         "123456",
//!         "YourSignature",
//!         vec!["123456".to_string()],
//!     );
//!     
//!     // Send SMS
//!     let response = client.send_sms(request).await?;
//!     println!("SMS sent successfully: {:?}", response);
//!     
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod error;
pub mod sms;

// Re-export main types for convenient usage
pub use crate::core::{Client, ClientProfile, Credential, HttpProfile};
pub use crate::error::{Result, TencentCloudError};
pub use crate::sms::{SendSmsRequest, SendSmsResponse, SendStatus};

/// Initialize the SDK (placeholder for future initialization needs)
pub fn init_api() {
    // Currently no initialization needed, but keeping for API compatibility
}

/// Shutdown the SDK (placeholder for future cleanup needs)
pub fn shutdown_api() {
    // Currently no cleanup needed, but keeping for API compatibility
}
