/*
 * SaaSus Auth API Schema
 *
 * スキーマ
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MfaConfiguration : MFAデバイス認証設定(MFA device authentication settings) ※ 未提供の機能のため、変更・保存はできません(This function is not yet provided, so it cannot be changed or saved.) 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MfaConfiguration {
    /// on: 全ユーザーがログイン時に適用(apply when all users log in) optional: MFA要素が有効になっている個別ユーザーに適用(apply to individual users with MFA factor enabled) ※ パラメータは現在optionalで固定となります。(The parameter is currently optional and fixed.) 
    #[serde(rename = "mfa_configuration")]
    pub mfa_configuration: MfaConfigurationEnum,
}

impl MfaConfiguration {
    /// MFAデバイス認証設定(MFA device authentication settings) ※ 未提供の機能のため、変更・保存はできません(This function is not yet provided, so it cannot be changed or saved.) 
    pub fn new(mfa_configuration: MfaConfigurationEnum) -> MfaConfiguration {
        MfaConfiguration {
            mfa_configuration,
        }
    }
}

/// on: 全ユーザーがログイン時に適用(apply when all users log in) optional: MFA要素が有効になっている個別ユーザーに適用(apply to individual users with MFA factor enabled) ※ パラメータは現在optionalで固定となります。(The parameter is currently optional and fixed.) 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MfaConfigurationEnum {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "optional")]
    Optional,
}

impl Default for MfaConfigurationEnum {
    fn default() -> MfaConfigurationEnum {
        Self::On
    }
}

