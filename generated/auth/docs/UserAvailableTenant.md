# UserAvailableTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** | テナント名(tenant name) | 
**completed_sign_up** | **bool** |  | 
**envs** | [**Vec<crate::models::UserAvailableEnv>**](UserAvailableEnv.md) | 環境情報、役割(ロール)情報(environmental info, role info) | 
**user_attribute** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | ユーザー追加属性(user additional attributes) | 
**back_office_staff_email** | **String** | バックオフィス担当者のメール(back office contact email) | 
**plan_id** | Option<**String**> |  | [optional]
**is_paid** | Option<**bool**> | テナントの支払い状況(tenant payment status)  ※ 現在はストライプ連携時のみ返却されます。Currently, it is returned only when stripe is linked.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


