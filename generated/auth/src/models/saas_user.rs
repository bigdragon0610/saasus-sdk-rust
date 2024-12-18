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
pub struct SaasUser {
    #[serde(rename = "id")]
    pub id: String,
    /// E-mail
    #[serde(rename = "email")]
    pub email: String,
    /// Attribute information 
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, serde_json::Value>,
}

impl SaasUser {
    pub fn new(id: String, email: String, attributes: ::std::collections::HashMap<String, serde_json::Value>) -> SaasUser {
        SaasUser {
            id,
            email,
            attributes,
        }
    }
}


