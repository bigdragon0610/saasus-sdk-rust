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
pub struct Credentials {
    /// IDトークン(ID token)
    #[serde(rename = "id_token")]
    pub id_token: String,
    /// アクセストークン(access token)
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// リフレッシュトークン(refresh token)
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl Credentials {
    pub fn new(id_token: String, access_token: String) -> Credentials {
        Credentials {
            id_token,
            access_token,
            refresh_token: None,
        }
    }
}


