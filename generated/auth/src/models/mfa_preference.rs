/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MfaPreference {
    /// MFAを有効にするか否か(enable MFA)
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// MFAの方法(enabledがtrueの場合は必須)(MFA method (required if enabled is true))
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
}

impl MfaPreference {
    pub fn new(enabled: bool) -> MfaPreference {
        MfaPreference {
            enabled,
            method: None,
        }
    }
}

/// MFAの方法(enabledがtrueの場合は必須)(MFA method (required if enabled is true))
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "softwareToken")]
    SoftwareToken,
}

impl Default for Method {
    fn default() -> Method {
        Self::SoftwareToken
    }
}

