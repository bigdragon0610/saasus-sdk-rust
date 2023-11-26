# PricingUnitForSave

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**upper_count** | **i32** | 上限値(upper limit) | 
**metering_unit_name** | **String** |  | 
**aggregate_usage** | Option<[**crate::models::AggregateUsage**](AggregateUsage.md)> |  | [optional]
**name** | **String** | 名前(name) | 
**display_name** | **String** | 表示名(display name) | 
**description** | **String** | 説明(description) | 
**r#type** | [**crate::models::UnitType**](UnitType.md) |  | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**tiers** | [**Vec<crate::models::PricingTier>**](PricingTier.md) |  | 
**unit_amount** | **i32** | 料金(price) | 
**recurring_interval** | [**crate::models::RecurringInterval**](RecurringInterval.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


