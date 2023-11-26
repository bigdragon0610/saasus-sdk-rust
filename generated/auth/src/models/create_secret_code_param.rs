/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSecretCodeParam {
    /// アクセストークン(access token)
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl CreateSecretCodeParam {
    pub fn new(access_token: String) -> CreateSecretCodeParam {
        CreateSecretCodeParam {
            access_token,
        }
    }
}


