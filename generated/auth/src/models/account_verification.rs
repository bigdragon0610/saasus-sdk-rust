/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountVerification : Account authentication settings ※ This function is not yet provided, so it cannot be changed or saved. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountVerification {
    /// code: verification code link: verification link ※ This function is not yet provided, so it cannot be changed or saved. 
    #[serde(rename = "verification_method")]
    pub verification_method: VerificationMethodEnum,
    /// email: e-mail sms: SMS smsOrEmail: email if SMS is not possible 
    #[serde(rename = "sending_to")]
    pub sending_to: SendingToEnum,
}

impl AccountVerification {
    /// Account authentication settings ※ This function is not yet provided, so it cannot be changed or saved. 
    pub fn new(verification_method: VerificationMethodEnum, sending_to: SendingToEnum) -> AccountVerification {
        AccountVerification {
            verification_method,
            sending_to,
        }
    }
}

/// code: verification code link: verification link ※ This function is not yet provided, so it cannot be changed or saved. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationMethodEnum {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "link")]
    Link,
}

impl Default for VerificationMethodEnum {
    fn default() -> VerificationMethodEnum {
        Self::Code
    }
}
/// email: e-mail sms: SMS smsOrEmail: email if SMS is not possible 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendingToEnum {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "smsOrEmail")]
    SmsOrEmail,
}

impl Default for SendingToEnum {
    fn default() -> SendingToEnum {
        Self::Email
    }
}

