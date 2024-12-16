# BasicInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain_name** | **String** | Domain Name | 
**is_dns_validated** | **bool** | DNS Record Verification Results | 
**certificate_dns_record** | [**crate::models::DnsRecord**](DnsRecord.md) |  | 
**cloud_front_dns_record** | [**crate::models::DnsRecord**](DnsRecord.md) |  | 
**dkim_dns_records** | [**Vec<crate::models::DnsRecord>**](DnsRecord.md) | DKIM DNS Records | 
**default_domain_name** | **String** | Default Domain Name | 
**from_email_address** | **String** | Sender Email for Authentication Email | 
**reply_email_address** | **String** | Reply-from email address of authentication email | 
**is_ses_sandbox_granted** | **bool** | SES sandbox release and Cognito SES configuration results | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


