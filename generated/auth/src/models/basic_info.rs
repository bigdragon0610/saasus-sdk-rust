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
pub struct BasicInfo {
    /// ドメイン名(Domain Name)
    #[serde(rename = "domain_name")]
    pub domain_name: String,
    /// DNSレコードの検証結果(DNS Record Verification Results)
    #[serde(rename = "is_dns_validated")]
    pub is_dns_validated: bool,
    #[serde(rename = "certificate_dns_record")]
    pub certificate_dns_record: Box<crate::models::DnsRecord>,
    #[serde(rename = "cloud_front_dns_record")]
    pub cloud_front_dns_record: Box<crate::models::DnsRecord>,
    /// DKIM DNS レコード(DKIM DNS Records)
    #[serde(rename = "dkim_dns_records")]
    pub dkim_dns_records: Vec<crate::models::DnsRecord>,
    /// デフォルトドメイン名(Default Domain Name)
    #[serde(rename = "default_domain_name")]
    pub default_domain_name: String,
    /// 認証メールの送信元メールアドレス(Sender Email for Authentication Email)
    #[serde(rename = "from_email_address")]
    pub from_email_address: String,
    /// 認証メールの返信元メールアドレス(Reply-from email address of authentication email)
    #[serde(rename = "reply_email_address")]
    pub reply_email_address: String,
    /// SESのサンドボックス解除及びCognitoのSES設定結果(SES sandbox release and Cognito SES configuration results)
    #[serde(rename = "is_ses_sandbox_granted")]
    pub is_ses_sandbox_granted: bool,
}

impl BasicInfo {
    pub fn new(domain_name: String, is_dns_validated: bool, certificate_dns_record: crate::models::DnsRecord, cloud_front_dns_record: crate::models::DnsRecord, dkim_dns_records: Vec<crate::models::DnsRecord>, default_domain_name: String, from_email_address: String, reply_email_address: String, is_ses_sandbox_granted: bool) -> BasicInfo {
        BasicInfo {
            domain_name,
            is_dns_validated,
            certificate_dns_record: Box::new(certificate_dns_record),
            cloud_front_dns_record: Box::new(cloud_front_dns_record),
            dkim_dns_records,
            default_domain_name,
            from_email_address,
            reply_email_address,
            is_ses_sandbox_granted,
        }
    }
}


