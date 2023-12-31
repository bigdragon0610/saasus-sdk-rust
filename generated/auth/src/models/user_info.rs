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
pub struct UserInfo {
    #[serde(rename = "id")]
    pub id: String,
    /// メールアドレス(E-mail)
    #[serde(rename = "email")]
    pub email: String,
    /// テナント情報(Tenant Info)
    #[serde(rename = "tenants")]
    pub tenants: Vec<crate::models::UserAvailableTenant>,
}

impl UserInfo {
    pub fn new(id: String, email: String, tenants: Vec<crate::models::UserAvailableTenant>) -> UserInfo {
        UserInfo {
            id,
            email,
            tenants,
        }
    }
}


