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
pub struct UpdateEnvParam {
    /// 環境名(env name)
    #[serde(rename = "name")]
    pub name: String,
    /// 環境表示名(env display name)
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl UpdateEnvParam {
    pub fn new(name: String) -> UpdateEnvParam {
        UpdateEnvParam {
            name,
            display_name: None,
        }
    }
}


