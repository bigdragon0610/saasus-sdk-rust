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
pub struct SaasUsers {
    #[serde(rename = "users")]
    pub users: Vec<crate::models::SaasUser>,
}

impl SaasUsers {
    pub fn new(users: Vec<crate::models::SaasUser>) -> SaasUsers {
        SaasUsers {
            users,
        }
    }
}


