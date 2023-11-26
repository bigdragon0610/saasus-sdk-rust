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
pub struct TenantIdentityProviders {
    #[serde(rename = "saml", skip_serializing_if = "Option::is_none")]
    pub saml: Option<Box<crate::models::TenantIdentityProvidersSaml>>,
}

impl TenantIdentityProviders {
    pub fn new() -> TenantIdentityProviders {
        TenantIdentityProviders {
            saml: None,
        }
    }
}


