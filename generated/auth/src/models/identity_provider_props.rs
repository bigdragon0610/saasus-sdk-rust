/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderProps {
    #[serde(rename = "application_id")]
    pub application_id: String,
    #[serde(rename = "application_secret")]
    pub application_secret: String,
    #[serde(rename = "approval_scope")]
    pub approval_scope: String,
    #[serde(rename = "is_button_hidden", skip_serializing_if = "Option::is_none")]
    pub is_button_hidden: Option<bool>,
}

impl IdentityProviderProps {
    pub fn new(application_id: String, application_secret: String, approval_scope: String) -> IdentityProviderProps {
        IdentityProviderProps {
            application_id,
            application_secret,
            approval_scope,
            is_button_hidden: None,
        }
    }
}


