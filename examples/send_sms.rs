//! Example: Send SMS using TencentCloud SMS SDK
//!
//! This example demonstrates how to:
//! - Create credentials
//! - Create a client
//! - Send SMS messages
//! - Handle responses and errors
//!
//! Before running this example, make sure to:
//! 1. Set environment variables:
//!    - TENCENTCLOUD_SECRET_ID
//!    - TENCENTCLOUD_SECRET_KEY
//! 2. Replace the placeholder values with your actual SMS configuration
//!
//! Usage:
//! ```
//! cargo run --example send_sms
//! ```

use tencentcloud_sms_sdk::{
    Client, ClientProfile, Credential, HttpProfile, SendSmsRequest, TencentCloudError,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("TencentCloud SMS SDK Example - Send SMS");
    println!("=====================================");

    // Example 1: Basic SMS sending
    println!("\n1. Basic SMS sending:");
    basic_send_sms().await?;

    // Example 2: SMS sending with custom configuration
    println!("\n2. SMS sending with custom configuration:");
    send_sms_with_config().await?;

    // Example 3: International SMS sending
    println!("\n3. International SMS sending:");
    send_international_sms().await?;

    // Example 4: Error handling
    println!("\n4. Error handling:");
    handle_errors().await?;

    Ok(())
}

/// Basic SMS sending example
async fn basic_send_sms() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials from environment variables
    let credential =
        Credential::from_env().map_err(|e| format!("Failed to load credentials: {}", e))?;

    // Create client
    let client = Client::new(credential, "ap-guangzhou");

    // Create SMS request
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()], // Replace with actual phone numbers
        "1400000000",                       // Replace with your SMS SDK App ID
        "123456",                           // Replace with your template ID
        "YourSignature",                    // Replace with your signature
        vec!["123456".to_string()],         // Template parameters
    );

    // Validate request
    if let Err(e) = request.validate() {
        println!("Request validation failed: {}", e);
        return Ok(());
    }

    // Send SMS
    match client.send_sms(request).await {
        Ok(response) => {
            println!("SMS sent successfully!");
            println!("Request ID: {}", response.request_id);
            println!("Total messages: {}", response.send_status_set.len());
            println!("Successful: {}", response.success_count());
            println!("Failed: {}", response.failed_count());
            println!("Total fee: {}", response.get_total_fee());

            // Print details for each phone number
            for status in &response.send_status_set {
                println!(
                    "Phone: {}, Status: {}, Message: {}",
                    status.phone_number, status.code, status.message
                );
            }
        }
        Err(e) => {
            println!("Failed to send SMS: {}", e.print_all());
        }
    }

    Ok(())
}

/// SMS sending with custom configuration
async fn send_sms_with_config() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credential =
        Credential::from_env().map_err(|e| format!("Failed to load credentials: {}", e))?;

    // Create custom HTTP profile
    let mut http_profile = HttpProfile::new();
    http_profile
        .set_req_timeout(30)
        .set_connect_timeout(30)
        .set_keep_alive(true);

    // Create client profile with custom HTTP profile
    let mut client_profile = ClientProfile::with_http_profile(http_profile);
    client_profile.set_debug(true); // Enable debug logging

    // Create client with custom profile
    let client = Client::with_profile(credential, "ap-guangzhou", client_profile);

    // Create SMS request with additional options
    let mut request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "YourSignature",
        vec!["123456".to_string()],
    );

    // Set additional options
    request
        .set_session_context("example_session_123")
        .set_extend_code("01");

    // Send SMS
    match client.send_sms(request).await {
        Ok(response) => {
            println!("SMS sent with custom config!");
            println!("Request ID: {}", response.request_id);

            // Check individual phone numbers
            for phone in &["+8613800000000"] {
                if response.check_phone_success(phone) {
                    println!("✓ {} - Success", phone);
                } else {
                    println!("✗ {} - Failed", phone);
                }
            }
        }
        Err(e) => {
            println!("Failed to send SMS: {}", e.print_all());
        }
    }

    Ok(())
}

/// International SMS sending example
async fn send_international_sms() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credential =
        Credential::from_env().map_err(|e| format!("Failed to load credentials: {}", e))?;

    // Create client
    let client = Client::new(credential, "ap-guangzhou");

    // Create international SMS request (no signature required)
    let request = SendSmsRequest::new_international(
        vec!["+1234567890".to_string()], // International phone number
        "1400000000",                    // SMS SDK App ID
        "123456",                        // International template ID
        vec!["123456".to_string()],      // Template parameters
    );

    // Send SMS
    match client.send_sms(request).await {
        Ok(response) => {
            println!("International SMS sent successfully!");
            println!("Request ID: {}", response.request_id);

            // Print status for each number
            for status in &response.send_status_set {
                println!(
                    "Phone: {}, Country: {}, Status: {}, Fee: {}",
                    status.phone_number, status.iso_code, status.code, status.fee
                );
            }
        }
        Err(e) => {
            println!("Failed to send international SMS: {}", e.print_all());
        }
    }

    Ok(())
}

/// Error handling examples
async fn handle_errors() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials with invalid values for demonstration
    let credential = Credential::new("invalid_id", "invalid_key", None);
    let client = Client::new(credential, "ap-guangzhou");

    // Create a valid request
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "YourSignature",
        vec!["123456".to_string()],
    );

    // Try to send SMS with invalid credentials
    match client.send_sms(request).await {
        Ok(_) => {
            println!("Unexpected success with invalid credentials");
        }
        Err(e) => {
            println!("Expected error occurred:");
            println!("Error type: {}", e);
            println!("Error details: {}", e.print_all());

            // Check specific error types
            if e.is_network_error() {
                println!("This is a network error");
            } else if let Some(code) = e.code() {
                println!("API error code: {}", code);

                // Handle specific error codes
                match code {
                    "UnauthorizedOperation.SmsSdkAppIdVerifyFail" => {
                        println!("Solution: Check your SMS SDK App ID");
                    }
                    "FailedOperation.SignatureIncorrectOrUnapproved" => {
                        println!("Solution: Check your SMS signature");
                    }
                    "FailedOperation.TemplateIncorrectOrUnapproved" => {
                        println!("Solution: Check your SMS template");
                    }
                    _ => {
                        println!("Unknown error code: {}", code);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Example helper function to demonstrate batch processing
#[allow(dead_code)]
async fn send_sms_batch() -> Result<(), Box<dyn std::error::Error>> {
    let credential = Credential::from_env()?;
    let client = Client::new(credential, "ap-guangzhou");

    // Send SMS to multiple phone numbers
    let phone_numbers = vec![
        "+8613800000000".to_string(),
        "+8613800000001".to_string(),
        "+8613800000002".to_string(),
    ];

    let request = SendSmsRequest::new(
        phone_numbers,
        "1400000000",
        "123456",
        "YourSignature",
        vec!["123456".to_string()],
    );

    match client.send_sms(request).await {
        Ok(response) => {
            println!("Batch SMS Results:");
            println!("Total: {}", response.send_status_set.len());
            println!("Success: {}", response.success_count());
            println!("Failed: {}", response.failed_count());

            // Print successful numbers
            let successful_numbers = response.get_successful_numbers();
            if !successful_numbers.is_empty() {
                println!("Successful numbers: {:?}", successful_numbers);
            }

            // Print failed numbers with reasons
            let failed_numbers = response.get_failed_numbers();
            if !failed_numbers.is_empty() {
                println!("Failed numbers:");
                for (phone, reason) in failed_numbers {
                    println!("  {} - {}", phone, reason);
                }
            }
        }
        Err(e) => {
            println!("Batch SMS failed: {}", e.print_all());
        }
    }

    Ok(())
}
