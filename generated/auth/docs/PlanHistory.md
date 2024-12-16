# PlanHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plan_id** | **String** |  | 
**plan_applied_at** | **i32** | Registration date | 
**tax_rate_id** | Option<**String**> |  | [optional]
**proration_behavior** | Option<[**crate::models::ProrationBehavior**](ProrationBehavior.md)> |  | [optional]
**delete_usage** | Option<**bool**> | If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


