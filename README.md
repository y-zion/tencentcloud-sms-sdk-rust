# TencentCloud SMS SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/tencentcloud-sms-sdk.svg)](https://crates.io/crates/tencentcloud-sms-sdk)
[![Documentation](https://docs.rs/tencentcloud-sms-sdk/badge.svg)](https://docs.rs/tencentcloud-sms-sdk)
[![License](https://img.shields.io/crates/l/tencentcloud-sms-sdk.svg)](LICENSE)

A Rust implementation of the TencentCloud SMS SDK, providing a comprehensive interface for sending SMS messages through TencentCloud's SMS service.

## Features

- **Complete SMS API Coverage**: Send SMS messages (verification codes, notifications, marketing)
- **Async/Await Support**: Built on `tokio` for high-performance async operations
- **Type Safety**: Strongly typed request/response models with validation
- **Error Handling**: Comprehensive error types with detailed error information
- **Authentication**: TC3-HMAC-SHA256 signature algorithm support
- **Multiple Regions**: Support for all TencentCloud regions
- **Domestic & International**: Support for both domestic and international SMS
- **Configurable**: Customizable HTTP profiles, timeouts, and proxy settings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tencentcloud-sms-sdk = "0.1.2"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use tencentcloud_sms_sdk::{Client, Credential, SendSmsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credential = Credential::new("your_secret_id", "your_secret_key", None);
    
    // Create client
    let client = Client::new(credential, "ap-guangzhou");
    
    // Create SMS request
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],  // Phone numbers
        "1400000000",                        // SMS SDK App ID
        "123456",                           // Template ID
        "YourSignature",                    // SMS signature
        vec!["123456".to_string()],         // Template parameters
    );
    
    // Send SMS
    let response = client.send_sms(request).await?;
    
    println!("SMS sent! Request ID: {}", response.request_id);
    println!("Success count: {}", response.success_count());
    println!("Failed count: {}", response.failed_count());
    
    Ok(())
}
```

### Using Environment Variables

```rust
use tencentcloud_sms_sdk::{Client, Credential, SendSmsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set environment variables:
    // export TENCENTCLOUD_SECRET_ID=your_secret_id
    // export TENCENTCLOUD_SECRET_KEY=your_secret_key
    
    let credential = Credential::from_env()?;
    let client = Client::new(credential, "ap-guangzhou");
    
    // ... rest of the code
    
    Ok(())
}
```

## Configuration

### Custom HTTP Profile

```rust
use tencentcloud_sms_sdk::{Client, ClientProfile, Credential, HttpProfile};

let credential = Credential::from_env()?;

// Create custom HTTP profile
let mut http_profile = HttpProfile::new();
http_profile
    .set_req_timeout(30)
    .set_connect_timeout(30)
    .set_keep_alive(true)
    .set_endpoint("sms.ap-guangzhou.tencentcloudapi.com");

// Create client profile
let client_profile = ClientProfile::with_http_profile(http_profile);

// Create client with custom profile
let client = Client::with_profile(credential, "ap-guangzhou", client_profile);
```

### Proxy Configuration

```rust
let mut http_profile = HttpProfile::new();
http_profile
    .set_proxy_host(Some("proxy.example.com"))
    .set_proxy_port(Some(8080));

let client_profile = ClientProfile::with_http_profile(http_profile);
let client = Client::with_profile(credential, "ap-guangzhou", client_profile);
```

## Examples

### Domestic SMS

```rust
let request = SendSmsRequest::new(
    vec!["+8613800000000".to_string()],
    "1400000000",
    "123456",
    "YourSignature",
    vec!["123456".to_string()],
);

let response = client.send_sms(request).await?;
```

### International SMS

```rust
let request = SendSmsRequest::new_international(
    vec!["+1234567890".to_string()],
    "1400000000",
    "123456",
    vec!["123456".to_string()],
);

let response = client.send_sms(request).await?;
```

### Batch SMS

```rust
let request = SendSmsRequest::new(
    vec![
        "+8613800000000".to_string(),
        "+8613800000001".to_string(),
        "+8613800000002".to_string(),
    ],
    "1400000000",
    "123456",
    "YourSignature",
    vec!["123456".to_string()],
);

let response = client.send_sms(request).await?;

// Check results
for status in &response.send_status_set {
    if status.is_success() {
        println!("✓ {} sent successfully", status.phone_number);
    } else {
        println!("✗ {} failed: {}", status.phone_number, status.message);
    }
}
```

### Advanced Request Configuration

```rust
let mut request = SendSmsRequest::new(
    vec!["+8613800000000".to_string()],
    "1400000000",
    "123456",
    "YourSignature",
    vec!["123456".to_string()],
);

// Set additional options
request
    .set_session_context("user_session_123")
    .set_extend_code("01")
    .set_sender_id("YourSenderID");

let response = client.send_sms(request).await?;
```

## Error Handling

The SDK provides comprehensive error handling with detailed error information:

```rust
use tencentcloud_sms_sdk::error::error_codes;

match client.send_sms(request).await {
    Ok(response) => {
        println!("Success: {}", response.request_id);
    }
    Err(e) => {
        println!("Error: {}", e.print_all());
        
        // Check specific error types
        if e.is_network_error() {
            println!("Network error occurred");
        } else if let Some(code) = e.code() {
            match code {
                error_codes::SIGNATURE_INCORRECT_OR_UNAPPROVED => {
                    println!("Please check your SMS signature");
                }
                error_codes::TEMPLATE_INCORRECT_OR_UNAPPROVED => {
                    println!("Please check your SMS template");
                }
                error_codes::SMS_SDK_APP_ID_VERIFY_FAIL => {
                    println!("Please check your SMS SDK App ID");
                }
                _ => {
                    println!("API error: {}", code);
                }
            }
        }
    }
}
```

## Response Handling

The `SendSmsResponse` provides various methods to check the results:

```rust
let response = client.send_sms(request).await?;

// Check overall success
if response.is_all_success() {
    println!("All messages sent successfully!");
} else {
    println!("Some messages failed to send");
}

// Get statistics
println!("Total: {}", response.send_status_set.len());
println!("Success: {}", response.success_count());
println!("Failed: {}", response.failed_count());
println!("Total fee: {}", response.get_total_fee());

// Check specific phone numbers
if response.check_phone_success("+8613800000000") {
    println!("Message to +8613800000000 was sent successfully");
}

// Get failed numbers with reasons
let failed_numbers = response.get_failed_numbers();
for (phone, reason) in failed_numbers {
    println!("Failed: {} - {}", phone, reason);
}
```

## Regions

The SDK supports all TencentCloud regions:

```rust
// China regions
let client = Client::new(credential, "ap-beijing");     // Beijing
let client = Client::new(credential, "ap-shanghai");    // Shanghai
let client = Client::new(credential, "ap-guangzhou");   // Guangzhou
let client = Client::new(credential, "ap-chengdu");     // Chengdu

// International regions
let client = Client::new(credential, "ap-singapore");   // Singapore
let client = Client::new(credential, "ap-seoul");       // Seoul
let client = Client::new(credential, "ap-tokyo");       // Tokyo
let client = Client::new(credential, "us-east-1");      // US East
let client = Client::new(credential, "eu-frankfurt");   // Europe
```

## Prerequisites

Before using the SDK, you need to:

1. **Create a TencentCloud account** and obtain your `SecretId` and `SecretKey`
2. **Enable SMS service** in the TencentCloud console
3. **Create SMS signatures** and get them approved
4. **Create SMS templates** and get them approved
5. **Get your SMS SDK App ID** from the SMS console

## Environment Variables

The SDK supports the following environment variables:

- `TENCENTCLOUD_SECRET_ID` or `TC_SECRET_ID`: Your TencentCloud Secret ID
- `TENCENTCLOUD_SECRET_KEY` or `TC_SECRET_KEY`: Your TencentCloud Secret Key
- `TENCENTCLOUD_TOKEN` or `TC_TOKEN`: Session token (for temporary credentials)

## Security Best Practices

1. **Never hardcode credentials** in your source code
2. **Use environment variables** or configuration files for credentials
3. **Implement proper error handling** to avoid credential leakage
4. **Use temporary credentials** when possible
5. **Regularly rotate** your access keys

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [https://docs.rs/tencentcloud-sms-sdk](https://docs.rs/tencentcloud-sms-sdk)
- **Issues**: [GitHub Issues](https://github.com/yourusername/tencentcloud-sms-sdk/issues)
- **TencentCloud SMS Documentation**: [https://cloud.tencent.com/document/product/382](https://cloud.tencent.com/document/product/382)

## Acknowledgments

This SDK is based on the [TencentCloud C++ SDK](https://github.com/TencentCloud/tencentcloud-sdk-cpp) implementation by [Cursor](https://cursor.com) and follows the same API patterns and structure for consistency across different language SDKs.