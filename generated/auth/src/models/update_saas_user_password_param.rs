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
pub struct UpdateSaasUserPasswordParam {
    /// Password
    #[serde(rename = "password")]
    pub password: String,
}

impl UpdateSaasUserPasswordParam {
    pub fn new(password: String) -> UpdateSaasUserPasswordParam {
        UpdateSaasUserPasswordParam {
            password,
        }
    }
}


