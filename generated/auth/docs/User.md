# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ユーザーID(User ID) | 
**tenant_id** | **String** |  | 
**tenant_name** | **String** | テナント名(Tenant Name) | 
**email** | **String** | メールアドレス(E-mail) | 
**attributes** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | 属性情報（SaaS 開発コンソールでユーザー属性定義を行い設定された情報を取得します）  Attribute information (Get information set by defining user attributes in the SaaS development console)  | 
**envs** | [**Vec<crate::models::UserAvailableEnv>**](UserAvailableEnv.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


