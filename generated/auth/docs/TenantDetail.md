# TenantDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**plan_id** | Option<**String**> |  | [optional]
**billing_info** | Option<[**crate::models::BillingInfo**](BillingInfo.md)> |  | [optional]
**name** | **String** | テナント名(tenant name) | 
**attributes** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | 属性情報(attribute info) | 
**back_office_staff_email** | **String** | 事務管理部門スタッフメールアドレス(administrative staff email address) | 
**next_plan_id** | Option<**String**> |  | [optional]
**using_next_plan_from** | Option<**i32**> | 次回料金プラン開始日時（stripe連携時、当月月初の0時（UTC）を指定すると当月月初開始のサブスクリプションを作成できます。ex. 2023年1月の場合は、1672531200 ） (Next billing plan start time (When using stripe, you can create a subscription that starts at the beginning of the current month by specifying 00:00 (UTC) at the beginning of the current month. Ex. 1672531200 for January 2023.))  | [optional]
**next_plan_tax_rate_id** | Option<**String**> |  | [optional]
**proration_behavior** | Option<[**crate::models::ProrationBehavior**](ProrationBehavior.md)> |  | [optional]
**delete_usage** | Option<**bool**> | stripe連携している場合で、プラン変更時に従量課金アイテムを削除するか設定できます。 プラン変更した場合に、現在のサブスクリプションに含まれる従量課金アイテムを全て削除して、従量課金アイテムに基づく請求の発生を止めることができます。 即時に記録している使用量がクリアされます。それらは復元できないため、delete_usageをtrueにしたプラン変更予約は取り消しできません。  If you have a stripe linkage,  you can set whether to delete pay-as-you-go items when changing plans. When you change plan, you can remove all pay-as-you-go items included in your current subscription to stop being billed based on pay-as-you-go items. The recorded usage is cleared immediately. Since it cannot be restored, please note that plan change reservations with delete_usage set to true cannot be canceled.  | [optional]
**plan_histories** | [**Vec<crate::models::PlanHistory>**](PlanHistory.md) | 料金プラン履歴 | 
**current_plan_period_start** | Option<**i32**> | 現在のプランの開始日時(current plan period start) | [optional]
**current_plan_period_end** | Option<**i32**> | 現在のプランの終了日時(current plan period end) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


