# PricingTieredUsageUnit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**upper_count** | **i32** | Upper limit | 
**metering_unit_name** | **String** | Metering unit name | 
**aggregate_usage** | Option<[**crate::models::AggregateUsage**](AggregateUsage.md)> |  | [optional]
**name** | **String** | Name | 
**display_name** | **String** | Display Name | 
**description** | **String** | Description | 
**r#type** | [**crate::models::UnitType**](UnitType.md) |  | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**tiers** | [**Vec<crate::models::PricingTier>**](PricingTier.md) |  | 
**id** | **String** | Universally Unique Identifier | 
**metering_unit_id** | **String** | Universally Unique Identifier | 
**recurring_interval** | [**crate::models::RecurringInterval**](RecurringInterval.md) |  | 
**used** | **bool** | Indicates if the unit is used | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


