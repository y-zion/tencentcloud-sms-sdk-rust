//! Integration tests for the TencentCloud SMS SDK

use tencentcloud_sms_sdk::{
    Client, ClientProfile, Credential, HttpProfile, SendSmsRequest, SendSmsResponse, SendStatus,
    TencentCloudError,
};

#[tokio::test]
async fn test_credential_creation() {
    let credential = Credential::new("test_id", "test_key", None);
    assert_eq!(credential.secret_id(), "test_id");
    assert_eq!(credential.secret_key(), "test_key");
    assert_eq!(credential.token(), None);
}

#[tokio::test]
async fn test_credential_with_token() {
    let credential = Credential::new("test_id", "test_key", Some("test_token"));
    assert_eq!(credential.secret_id(), "test_id");
    assert_eq!(credential.secret_key(), "test_key");
    assert_eq!(credential.token(), Some("test_token"));
}

#[tokio::test]
async fn test_client_creation() {
    let credential = Credential::new("test_id", "test_key", None);
    let client = Client::new(credential, "ap-guangzhou");
    assert_eq!(client.region(), "ap-guangzhou");
    assert_eq!(client.service(), "sms");
}

#[tokio::test]
async fn test_send_sms_request_creation() {
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );

    assert_eq!(request.phone_number_set, vec!["+8613800000000"]);
    assert_eq!(request.sms_sdk_app_id, "1400000000");
    assert_eq!(request.template_id, "123456");
    assert_eq!(request.sign_name, Some("TestSignature".to_string()));
    assert_eq!(request.template_param_set, Some(vec!["123456".to_string()]));
}

#[tokio::test]
async fn test_send_sms_request_international() {
    let request = SendSmsRequest::new_international(
        vec!["+1234567890".to_string()],
        "1400000000",
        "123456",
        vec!["123456".to_string()],
    );

    assert_eq!(request.phone_number_set, vec!["+1234567890"]);
    assert_eq!(request.sms_sdk_app_id, "1400000000");
    assert_eq!(request.template_id, "123456");
    assert_eq!(request.sign_name, None);
    assert_eq!(request.template_param_set, Some(vec!["123456".to_string()]));
}

#[tokio::test]
async fn test_send_sms_request_validation() {
    // Valid request
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );
    assert!(request.validate().is_ok());

    // Empty phone number set
    let request = SendSmsRequest::new(
        vec![],
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );
    assert!(request.validate().is_err());

    // Too many phone numbers
    let phone_numbers = (0..201).map(|i| format!("+861380000{:04}", i)).collect();
    let request = SendSmsRequest::new(
        phone_numbers,
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );
    assert!(request.validate().is_err());

    // Empty SMS SDK App ID
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );
    assert!(request.validate().is_err());
}

#[tokio::test]
async fn test_http_profile_configuration() {
    let mut http_profile = HttpProfile::new();
    http_profile
        .set_req_timeout(30)
        .set_connect_timeout(30)
        .set_keep_alive(true)
        .set_endpoint("custom.endpoint.com")
        .set_proxy_host(Some("proxy.example.com"))
        .set_proxy_port(Some(8080));

    assert_eq!(http_profile.req_timeout, 30);
    assert_eq!(http_profile.connect_timeout, 30);
    assert!(http_profile.keep_alive);
    assert_eq!(http_profile.endpoint, "custom.endpoint.com");
    assert_eq!(
        http_profile.proxy_host,
        Some("proxy.example.com".to_string())
    );
    assert_eq!(http_profile.proxy_port, Some(8080));
}

#[tokio::test]
async fn test_client_profile_configuration() {
    let mut client_profile = ClientProfile::new();
    client_profile
        .set_sign_method("HmacSHA1")
        .set_api_version("2019-07-11")
        .set_language("zh-CN")
        .set_debug(true);

    assert_eq!(client_profile.sign_method, "HmacSHA1");
    assert_eq!(client_profile.api_version, "2019-07-11");
    assert_eq!(client_profile.language, "zh-CN");
    assert!(client_profile.debug);
}

#[tokio::test]
async fn test_send_status() {
    let status = SendStatus {
        serial_no: "12345".to_string(),
        phone_number: "+8613800000000".to_string(),
        fee: 1,
        session_context: "test".to_string(),
        code: "Ok".to_string(),
        message: "Success".to_string(),
        iso_code: "CN".to_string(),
    };

    assert!(status.is_success());
    assert_eq!(status.get_status_description(), "Success");

    let failed_status = SendStatus {
        serial_no: "12345".to_string(),
        phone_number: "+8613800000000".to_string(),
        fee: 0,
        session_context: "test".to_string(),
        code: "InvalidParameterValue.IncorrectPhoneNumber".to_string(),
        message: "Invalid phone number".to_string(),
        iso_code: "CN".to_string(),
    };

    assert!(!failed_status.is_success());
    assert_eq!(
        failed_status.get_status_description(),
        "Invalid phone number format"
    );
}

#[tokio::test]
async fn test_send_sms_response() {
    let response = SendSmsResponse {
        send_status_set: vec![
            SendStatus {
                serial_no: "12345".to_string(),
                phone_number: "+8613800000000".to_string(),
                fee: 1,
                session_context: "test".to_string(),
                code: "Ok".to_string(),
                message: "Success".to_string(),
                iso_code: "CN".to_string(),
            },
            SendStatus {
                serial_no: "12346".to_string(),
                phone_number: "+8613800000001".to_string(),
                fee: 0,
                session_context: "test".to_string(),
                code: "InvalidParameterValue.IncorrectPhoneNumber".to_string(),
                message: "Invalid phone number".to_string(),
                iso_code: "CN".to_string(),
            },
        ],
        request_id: "test-request-id".to_string(),
    };

    assert!(!response.is_all_success());
    assert_eq!(response.success_count(), 1);
    assert_eq!(response.failed_count(), 1);
    assert_eq!(response.get_total_fee(), 1);

    let successful_numbers = response.get_successful_numbers();
    assert_eq!(successful_numbers, vec!["+8613800000000"]);

    let failed_numbers = response.get_failed_numbers();
    assert_eq!(failed_numbers.len(), 1);
    assert_eq!(failed_numbers[0].0, "+8613800000001");

    assert!(response.check_phone_success("+8613800000000"));
    assert!(!response.check_phone_success("+8613800000001"));
}

#[tokio::test]
async fn test_error_handling() {
    let error = TencentCloudError::api("TestError", "Test error message");
    assert_eq!(error.code(), Some("TestError"));
    assert!(error.is_api_error("TestError"));
    assert!(!error.is_network_error());
    assert!(!error.is_timeout_error());

    let error_with_request_id = TencentCloudError::api_with_request_id(
        "TestError",
        "Test error message",
        Some("test-request-id"),
    );
    assert_eq!(error_with_request_id.request_id(), Some("test-request-id"));
}

#[tokio::test]
async fn test_client_with_profile() {
    let credential = Credential::new("test_id", "test_key", None);
    let mut http_profile = HttpProfile::new();
    http_profile.set_req_timeout(30);
    let client_profile = ClientProfile::with_http_profile(http_profile);
    let client = Client::with_profile(credential, "ap-guangzhou", client_profile);

    assert_eq!(client.region(), "ap-guangzhou");
    assert_eq!(client.profile().get_http_profile().req_timeout, 30);
}

#[tokio::test]
async fn test_credential_validation() {
    let credential = Credential::new("test_id", "test_key", None);
    assert!(credential.validate().is_ok());

    let invalid_credential = Credential::new("", "test_key", None);
    assert!(invalid_credential.validate().is_err());

    let invalid_credential = Credential::new("test_id", "", None);
    assert!(invalid_credential.validate().is_err());
}

// Test that demonstrates the usage pattern similar to the C++ SDK
#[tokio::test]
async fn test_cpp_like_usage_pattern() {
    // Similar to C++ SDK example structure
    let credential = Credential::new("test_id", "test_key", None);

    // Create HTTP profile
    let mut http_profile = HttpProfile::new();
    http_profile.set_keep_alive(true);
    http_profile.set_endpoint("sms.tencentcloudapi.com");
    http_profile.set_req_timeout(30);
    http_profile.set_connect_timeout(30);

    // Create client profile
    let client_profile = ClientProfile::with_http_profile(http_profile);

    // Create client
    let client = Client::with_profile(credential, "ap-guangzhou", client_profile);

    // Create request
    let request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );

    // Validate request
    assert!(request.validate().is_ok());

    // This would be where we make the actual API call in a real scenario
    // let response = client.send_sms(request).await?;
}

#[test]
fn test_library_exports() {
    // Test that all main types are properly exported
    let _credential = Credential::new("test", "test", None);
    let _http_profile = HttpProfile::new();
    let _client_profile = ClientProfile::new();
    let _request = SendSmsRequest::new(
        vec!["+8613800000000".to_string()],
        "1400000000",
        "123456",
        "TestSignature",
        vec!["123456".to_string()],
    );

    // Test error types
    let _error = TencentCloudError::api("TestError", "Test message");

    // If this compiles, it means all exports are working correctly
    assert!(true);
}
