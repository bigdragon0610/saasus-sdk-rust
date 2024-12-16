/*
 * SaaSus Auth API Schema
 *
 * Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    #[serde(rename = "SAML")]
    Saml,

}

impl ToString for ProviderType {
    fn to_string(&self) -> String {
        match self {
            Self::Saml => String::from("SAML"),
        }
    }
}

impl Default for ProviderType {
    fn default() -> ProviderType {
        Self::Saml
    }
}




