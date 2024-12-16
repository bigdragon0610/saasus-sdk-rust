/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityProviderConfiguration : This information is required to set up sign-in using an external identity provider. It cannot be changed. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderConfiguration {
    /// domain
    #[serde(rename = "domain")]
    pub domain: String,
    /// redirect URL
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    /// entity ID
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// reply URL
    #[serde(rename = "reply_url")]
    pub reply_url: String,
}

impl IdentityProviderConfiguration {
    /// This information is required to set up sign-in using an external identity provider. It cannot be changed. 
    pub fn new(domain: String, redirect_url: String, entity_id: String, reply_url: String) -> IdentityProviderConfiguration {
        IdentityProviderConfiguration {
            domain,
            redirect_url,
            entity_id,
            reply_url,
        }
    }
}


