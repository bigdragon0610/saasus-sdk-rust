/*
 * SaaSus Communication API Schema
 *
 * SaaSus Communication API Schema
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Votes {
    #[serde(rename = "users")]
    pub users: Vec<crate::models::User>,
    #[serde(rename = "count")]
    pub count: i32,
}

impl Votes {
    pub fn new(users: Vec<crate::models::User>, count: i32) -> Votes {
        Votes {
            users,
            count,
        }
    }
}


