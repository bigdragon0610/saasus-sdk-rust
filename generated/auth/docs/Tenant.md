# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | tenant name | 
**attributes** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | attribute info | 
**back_office_staff_email** | **String** | administrative staff email address | 
**next_plan_id** | Option<**String**> |  | [optional]
**using_next_plan_from** | Option<**i32**> | Next billing plan start time (When using stripe, you can create a subscription that starts at the beginning of the current month by specifying 00:00 (UTC) at the beginning of the current month. Ex. 1672531200 for January 2023.)  | [optional]
**next_plan_tax_rate_id** | Option<**String**> |  | [optional]
**proration_behavior** | Option<[**crate::models::ProrationBehavior**](ProrationBehavior.md)> |  | [optional]
**delete_usage** | Option<**bool**> | If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled.  | [optional]
**plan_histories** | [**Vec<crate::models::PlanHistory>**](PlanHistory.md) | Plan History | 
**id** | **String** |  | 
**plan_id** | Option<**String**> |  | [optional]
**billing_info** | Option<[**crate::models::BillingInfo**](BillingInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


