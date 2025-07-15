//! SMS service models and data structures

use serde::{Deserialize, Serialize};

/// Request structure for sending SMS
#[derive(Debug, Clone, Serialize)]
pub struct SendSmsRequest {
    /// List of phone numbers to send SMS to
    /// Format: +[country code][phone number]
    /// Example: +8613800000000
    /// Maximum 200 phone numbers per request
    #[serde(rename = "PhoneNumberSet")]
    pub phone_number_set: Vec<String>,

    /// SMS SDK App ID
    /// You can view it in the SMS console
    #[serde(rename = "SmsSdkAppId")]
    pub sms_sdk_app_id: String,

    /// Template ID
    /// You must use an approved template ID
    #[serde(rename = "TemplateId")]
    pub template_id: String,

    /// SMS signature content
    /// You must use an approved signature
    /// Required for domestic SMS, optional for international SMS
    #[serde(rename = "SignName", skip_serializing_if = "Option::is_none")]
    pub sign_name: Option<String>,

    /// Template parameters
    /// The number of parameters must match the template variables
    #[serde(rename = "TemplateParamSet", skip_serializing_if = "Option::is_none")]
    pub template_param_set: Option<Vec<String>>,

    /// SMS extension code
    /// Default is not enabled
    #[serde(rename = "ExtendCode", skip_serializing_if = "Option::is_none")]
    pub extend_code: Option<String>,

    /// User session context
    /// Server will return this as-is
    #[serde(rename = "SessionContext", skip_serializing_if = "Option::is_none")]
    pub session_context: Option<String>,

    /// SenderId for international SMS
    /// Required for international SMS with independent SenderId
    #[serde(rename = "SenderId", skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

impl SendSmsRequest {
    /// Create a new SendSmsRequest with required parameters
    ///
    /// # Arguments
    ///
    /// * `phone_number_set` - List of phone numbers
    /// * `sms_sdk_app_id` - SMS SDK App ID
    /// * `template_id` - Template ID
    /// * `sign_name` - SMS signature
    /// * `template_param_set` - Template parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tencentcloud_sms_sdk::SendSmsRequest;
    ///
    /// let request = SendSmsRequest::new(
    ///     vec!["+8613800000000".to_string()],
    ///     "1400000000",
    ///     "123456",
    ///     "YourSignature",
    ///     vec!["123456".to_string()],
    /// );
    /// ```
    pub fn new<S: Into<String>>(
        phone_number_set: Vec<String>,
        sms_sdk_app_id: S,
        template_id: S,
        sign_name: S,
        template_param_set: Vec<String>,
    ) -> Self {
        Self {
            phone_number_set,
            sms_sdk_app_id: sms_sdk_app_id.into(),
            template_id: template_id.into(),
            sign_name: Some(sign_name.into()),
            template_param_set: if template_param_set.is_empty() {
                None
            } else {
                Some(template_param_set)
            },
            extend_code: None,
            session_context: None,
            sender_id: None,
        }
    }

    /// Create a new SendSmsRequest for international SMS
    pub fn new_international<S: Into<String>>(
        phone_number_set: Vec<String>,
        sms_sdk_app_id: S,
        template_id: S,
        template_param_set: Vec<String>,
    ) -> Self {
        Self {
            phone_number_set,
            sms_sdk_app_id: sms_sdk_app_id.into(),
            template_id: template_id.into(),
            sign_name: None,
            template_param_set: if template_param_set.is_empty() {
                None
            } else {
                Some(template_param_set)
            },
            extend_code: None,
            session_context: None,
            sender_id: None,
        }
    }

    /// Set the SMS signature
    pub fn set_sign_name<S: Into<String>>(&mut self, sign_name: S) -> &mut Self {
        self.sign_name = Some(sign_name.into());
        self
    }

    /// Set template parameters
    pub fn set_template_param_set(&mut self, params: Vec<String>) -> &mut Self {
        self.template_param_set = if params.is_empty() {
            None
        } else {
            Some(params)
        };
        self
    }

    /// Set extension code
    pub fn set_extend_code<S: Into<String>>(&mut self, extend_code: S) -> &mut Self {
        self.extend_code = Some(extend_code.into());
        self
    }

    /// Set session context
    pub fn set_session_context<S: Into<String>>(&mut self, session_context: S) -> &mut Self {
        self.session_context = Some(session_context.into());
        self
    }

    /// Set sender ID for international SMS
    pub fn set_sender_id<S: Into<String>>(&mut self, sender_id: S) -> &mut Self {
        self.sender_id = Some(sender_id.into());
        self
    }

    /// Validate the request parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.phone_number_set.is_empty() {
            return Err("Phone number set cannot be empty".to_string());
        }

        if self.phone_number_set.len() > 200 {
            return Err("Phone number set cannot exceed 200 numbers".to_string());
        }

        if self.sms_sdk_app_id.is_empty() {
            return Err("SMS SDK App ID cannot be empty".to_string());
        }

        if self.template_id.is_empty() {
            return Err("Template ID cannot be empty".to_string());
        }

        // Validate phone number format
        for phone in &self.phone_number_set {
            if !phone.starts_with('+') && !phone.starts_with("0086") && !phone.starts_with("86") {
                if phone.len() != 11 {
                    return Err(format!("Invalid phone number format: {}", phone));
                }
            }
        }

        Ok(())
    }
}

/// SMS sending status information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendStatus {
    /// Serial number returned by the SMS sending API
    #[serde(rename = "SerialNo")]
    pub serial_no: String,

    /// Phone number
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,

    /// Number of billable SMS messages
    #[serde(rename = "Fee")]
    pub fee: i32,

    /// User session context
    #[serde(rename = "SessionContext", default)]
    pub session_context: String,

    /// SMS delivery status code
    #[serde(rename = "Code")]
    pub code: String,

    /// SMS delivery status message
    #[serde(rename = "Message")]
    pub message: String,

    /// Country/region code
    #[serde(rename = "IsoCode")]
    pub iso_code: String,
}

impl SendStatus {
    /// Check if the SMS was sent successfully
    pub fn is_success(&self) -> bool {
        self.code == "Ok"
    }

    /// Get a human-readable status description
    pub fn get_status_description(&self) -> &str {
        match self.code.as_str() {
            "Ok" => "Success",
            "InvalidParameterValue.IncorrectPhoneNumber" => "Invalid phone number format",
            "FailedOperation.SignatureIncorrectOrUnapproved" => "Signature incorrect or unapproved",
            "FailedOperation.TemplateIncorrectOrUnapproved" => "Template incorrect or unapproved",
            "FailedOperation.InsufficientBalanceInSmsPackage" => "Insufficient balance",
            "LimitExceeded.PhoneNumberCountLimit" => "Phone number count limit exceeded",
            "LimitExceeded.DeliveryFrequencyLimit" => "Delivery frequency limit exceeded",
            _ => "Unknown status",
        }
    }
}

/// Response structure for sending SMS
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendSmsResponse {
    /// SMS sending status list
    #[serde(rename = "SendStatusSet")]
    pub send_status_set: Vec<SendStatus>,

    /// Unique request ID
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

impl SendSmsResponse {
    /// Check if all SMS messages were sent successfully
    pub fn is_all_success(&self) -> bool {
        self.send_status_set
            .iter()
            .all(|status| status.is_success())
    }

    /// Get the count of successfully sent messages
    pub fn success_count(&self) -> usize {
        self.send_status_set
            .iter()
            .filter(|status| status.is_success())
            .count()
    }

    /// Get the count of failed messages
    pub fn failed_count(&self) -> usize {
        self.send_status_set
            .iter()
            .filter(|status| !status.is_success())
            .count()
    }

    /// Get failed phone numbers and their error messages
    pub fn get_failed_numbers(&self) -> Vec<(String, String)> {
        self.send_status_set
            .iter()
            .filter(|status| !status.is_success())
            .map(|status| (status.phone_number.clone(), status.message.clone()))
            .collect()
    }

    /// Get successful phone numbers
    pub fn get_successful_numbers(&self) -> Vec<String> {
        self.send_status_set
            .iter()
            .filter(|status| status.is_success())
            .map(|status| status.phone_number.clone())
            .collect()
    }

    /// Check if a specific phone number was sent successfully
    pub fn check_phone_success(&self, phone_number: &str) -> bool {
        self.send_status_set
            .iter()
            .find(|status| status.phone_number == phone_number)
            .map(|status| status.is_success())
            .unwrap_or(false)
    }

    /// Get status for a specific phone number
    pub fn get_phone_status(&self, phone_number: &str) -> Option<&SendStatus> {
        self.send_status_set
            .iter()
            .find(|status| status.phone_number == phone_number)
    }

    /// Get total fee for all sent messages
    pub fn get_total_fee(&self) -> i32 {
        self.send_status_set.iter().map(|status| status.fee).sum()
    }

    /// Convert to JSON string
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_sms_request_creation() {
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

    #[test]
    fn test_send_sms_request_international() {
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

    #[test]
    fn test_send_sms_request_validation() {
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

    #[test]
    fn test_send_status() {
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

    #[test]
    fn test_send_sms_response() {
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
}
