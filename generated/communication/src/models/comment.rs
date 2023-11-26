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
pub struct Comment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "body")]
    pub body: String,
}

impl Comment {
    pub fn new(id: String, user_id: String, created_at: i32, body: String) -> Comment {
        Comment {
            id,
            user_id,
            created_at,
            body,
        }
    }
}


