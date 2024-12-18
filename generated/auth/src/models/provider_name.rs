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
pub enum ProviderName {
    #[serde(rename = "Google")]
    Google,

}

impl ToString for ProviderName {
    fn to_string(&self) -> String {
        match self {
            Self::Google => String::from("Google"),
        }
    }
}

impl Default for ProviderName {
    fn default() -> ProviderName {
        Self::Google
    }
}




