/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Role : 役割(ロール)情報(role info)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    /// 役割(ロール)名(role name)
    #[serde(rename = "role_name")]
    pub role_name: String,
    /// 役割(ロール)表示名(role display name)
    #[serde(rename = "display_name")]
    pub display_name: String,
}

impl Role {
    /// 役割(ロール)情報(role info)
    pub fn new(role_name: String, display_name: String) -> Role {
        Role {
            role_name,
            display_name,
        }
    }
}


