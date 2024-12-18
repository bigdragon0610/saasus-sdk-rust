/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecaptchaProps : reCAPTCHA authentication settings ※ This function is not yet provided, so it cannot be changed or saved. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecaptchaProps {
    /// site key
    #[serde(rename = "site_key")]
    pub site_key: String,
    /// secret key
    #[serde(rename = "secret_key")]
    pub secret_key: String,
}

impl RecaptchaProps {
    /// reCAPTCHA authentication settings ※ This function is not yet provided, so it cannot be changed or saved. 
    pub fn new(site_key: String, secret_key: String) -> RecaptchaProps {
        RecaptchaProps {
            site_key,
            secret_key,
        }
    }
}


