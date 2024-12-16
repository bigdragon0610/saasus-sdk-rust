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
pub struct Roles {
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::Role>,
}

impl Roles {
    pub fn new(roles: Vec<crate::models::Role>) -> Roles {
        Roles {
            roles,
        }
    }
}


