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
pub struct User {
    /// User ID
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    /// Tenant Name
    #[serde(rename = "tenant_name")]
    pub tenant_name: String,
    /// E-mail
    #[serde(rename = "email")]
    pub email: String,
    /// Attribute information (Get information set by defining user attributes in the SaaS development console) 
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "envs")]
    pub envs: Vec<crate::models::UserAvailableEnv>,
}

impl User {
    pub fn new(id: String, tenant_id: String, tenant_name: String, email: String, attributes: ::std::collections::HashMap<String, serde_json::Value>, envs: Vec<crate::models::UserAvailableEnv>) -> User {
        User {
            id,
            tenant_id,
            tenant_name,
            email,
            attributes,
            envs,
        }
    }
}


