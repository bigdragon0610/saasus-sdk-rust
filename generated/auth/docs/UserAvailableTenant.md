# UserAvailableTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** | Tenant Name | 
**completed_sign_up** | **bool** |  | 
**envs** | [**Vec<crate::models::UserAvailableEnv>**](UserAvailableEnv.md) | environmental info, role info | 
**user_attribute** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | user additional attributes | 
**back_office_staff_email** | **String** | back office contact email | 
**plan_id** | Option<**String**> |  | [optional]
**is_paid** | Option<**bool**> | tenant payment status ※ Currently, it is returned only when stripe is linked.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


