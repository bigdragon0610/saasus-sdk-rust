/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ValidateInvitationParam : 既存ユーザーの場合はアクセストークン、新規ユーザーの場合はメールアドレスとパスワードが必須です。  Access token is required for existing users, and email and password is required for new users. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateInvitationParam {
    /// 招待されたユーザーのアクセストークン(access token of the invited user)
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// 招待されたユーザーのメールアドレス(email address of the invited user)
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 招待されたユーザーのパスワード(password of the invited user)
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ValidateInvitationParam {
    /// 既存ユーザーの場合はアクセストークン、新規ユーザーの場合はメールアドレスとパスワードが必須です。  Access token is required for existing users, and email and password is required for new users. 
    pub fn new() -> ValidateInvitationParam {
        ValidateInvitationParam {
            access_token: None,
            email: None,
            password: None,
        }
    }
}


