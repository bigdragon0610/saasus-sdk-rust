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
pub struct CreateVoteUserParam {
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl CreateVoteUserParam {
    pub fn new(user_id: String) -> CreateVoteUserParam {
        CreateVoteUserParam {
            user_id,
        }
    }
}


