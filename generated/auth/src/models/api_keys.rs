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
pub struct ApiKeys {
    /// API Key
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<String>,
}

impl ApiKeys {
    pub fn new(api_keys: Vec<String>) -> ApiKeys {
        ApiKeys {
            api_keys,
        }
    }
}


