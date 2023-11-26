/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentityProviderConfiguration : 外部IDプロバイダを利用したサインインの設定をするために必要な情報です。(This information is required to set up sign-in using an external identity provider.) 変更はできません。(It cannot be changed.) 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderConfiguration {
    /// ドメイン(domain)
    #[serde(rename = "domain")]
    pub domain: String,
    /// リダイレクトURL(redirect URL)
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    /// 識別子(entity ID)
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// 応答URL(reply URL)
    #[serde(rename = "reply_url")]
    pub reply_url: String,
}

impl IdentityProviderConfiguration {
    /// 外部IDプロバイダを利用したサインインの設定をするために必要な情報です。(This information is required to set up sign-in using an external identity provider.) 変更はできません。(It cannot be changed.) 
    pub fn new(domain: String, redirect_url: String, entity_id: String, reply_url: String) -> IdentityProviderConfiguration {
        IdentityProviderConfiguration {
            domain,
            redirect_url,
            entity_id,
            reply_url,
        }
    }
}


