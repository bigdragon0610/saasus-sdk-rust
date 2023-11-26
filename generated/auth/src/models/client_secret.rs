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
pub struct ClientSecret {
    /// クライアントシークレット(client secret)
    #[serde(rename = "client_secret")]
    pub client_secret: String,
}

impl ClientSecret {
    pub fn new(client_secret: String) -> ClientSecret {
        ClientSecret {
            client_secret,
        }
    }
}


