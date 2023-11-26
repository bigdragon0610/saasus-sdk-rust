# BasicInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_name** | **String** | ドメイン名(Domain Name) | 
**is_dns_validated** | **bool** | DNSレコードの検証結果(DNS Record Verification Results) | 
**certificate_dns_record** | [**crate::models::DnsRecord**](DnsRecord.md) |  | 
**cloud_front_dns_record** | [**crate::models::DnsRecord**](DnsRecord.md) |  | 
**dkim_dns_records** | [**Vec<crate::models::DnsRecord>**](DnsRecord.md) | DKIM DNS レコード(DKIM DNS Records) | 
**default_domain_name** | **String** | デフォルトドメイン名(Default Domain Name) | 
**from_email_address** | **String** | 認証メールの送信元メールアドレス(Sender Email for Authentication Email) | 
**reply_email_address** | **String** | 認証メールの返信元メールアドレス(Reply-from email address of authentication email) | 
**is_ses_sandbox_granted** | **bool** | SESのサンドボックス解除及びCognitoのSES設定結果(SES sandbox release and Cognito SES configuration results) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


