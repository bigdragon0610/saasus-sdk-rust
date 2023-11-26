# PricingTieredUsageUnitForSave

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | 名前(name) | 
**display_name** | **String** | 表示名(display name) | 
**description** | **String** | 説明(description) | 
**r#type** | [**crate::models::UnitType**](UnitType.md) |  | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**tiers** | [**Vec<crate::models::PricingTier>**](PricingTier.md) |  | 
**upper_count** | **i32** | 上限値(upper limit) | 
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | 
**aggregate_usage** | Option<[**crate::models::AggregateUsage**](AggregateUsage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


