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
pub struct ConfirmSignUpWithAwsMarketplaceParam {
    /// Tenant name
    #[serde(rename = "tenant_name", skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    /// Access token
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Registration Token
    #[serde(rename = "registration_token")]
    pub registration_token: String,
}

impl ConfirmSignUpWithAwsMarketplaceParam {
    pub fn new(access_token: String, registration_token: String) -> ConfirmSignUpWithAwsMarketplaceParam {
        ConfirmSignUpWithAwsMarketplaceParam {
            tenant_name: None,
            access_token,
            registration_token,
        }
    }
}


