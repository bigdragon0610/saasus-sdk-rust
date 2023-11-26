# \PricingUnitsApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pricing_unit**](PricingUnitsApi.md#create_pricing_unit) | **Post** /units | プライシングユニットを作成(Create Pricing Unit)
[**delete_pricing_unit**](PricingUnitsApi.md#delete_pricing_unit) | **Delete** /units/{pricing_unit_id} | プライシングユニットを削除(Delete Pricing Unit)
[**get_pricing_unit**](PricingUnitsApi.md#get_pricing_unit) | **Get** /units/{pricing_unit_id} | プライシングユニットを取得(Get Pricing Unit)
[**get_pricing_units**](PricingUnitsApi.md#get_pricing_units) | **Get** /units | プライシングユニットの一覧を取得(Get Pricing Units)
[**update_pricing_unit**](PricingUnitsApi.md#update_pricing_unit) | **Patch** /units/{pricing_unit_id} | プライシングユニットを更新(Update Pricing Unit)



## create_pricing_unit

> crate::models::PricingUnit create_pricing_unit(body)
プライシングユニットを作成(Create Pricing Unit)

プライシングユニットを作成します。  Create a pricing unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::PricingUnitForSave**> |  |  |

### Return type

[**crate::models::PricingUnit**](PricingUnit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pricing_unit

> delete_pricing_unit(pricing_unit_id)
プライシングユニットを削除(Delete Pricing Unit)

プライシングユニットを削除します。  Delete a pricing unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pricing_unit_id** | **String** | ユニットID(unit id) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_unit

> crate::models::PricingUnit get_pricing_unit(pricing_unit_id)
プライシングユニットを取得(Get Pricing Unit)

プライシングユニットを取得します。  Get a pricing unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pricing_unit_id** | **String** | ユニットID(unit id) | [required] |

### Return type

[**crate::models::PricingUnit**](PricingUnit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_units

> crate::models::PricingUnits get_pricing_units()
プライシングユニットの一覧を取得(Get Pricing Units)

料金のベースとなる最小の計測単位を取得します。 「固定ユニット」(type=fixed)は基本料金などの月額固定料金の単位、 「使用量ユニット」(type=usage)はユーザ数課金などの１単位あたりごとに料金が発生する単位、 「段階ユニット」(type=tiered)は携帯電話の段階的パケット料金のように利用量の段階ごとに一定の料金の単位、 「段階的使用量ユニット」(type=tiered_usage)はボリュームディスカウントのように利用量に応じて１単位あたりの料金が変化していく単位、となります。  Gets the smallest unit of measure on which the charges are based. \"Fixed Unit\" (type=fixed) is a unit of a monthly fixed charge such as a basic charge, \"Usage Unit\" (type=usage) is a unit in which a charge is generated per unit such as billing for the number of users, \"Tiered Unit\" (type = tiered) is a fixed charge unit for each tier of usage, such as the tiered packet charge for mobile phones, \"Tiered Usage Unit\" (type=tiered_usage) is a unit where the charge per unit changes according to the usage amount, such as a volume discount. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PricingUnits**](PricingUnits.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pricing_unit

> update_pricing_unit(pricing_unit_id, body)
プライシングユニットを更新(Update Pricing Unit)

プライシングユニット情報を更新します。  Update pricing unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pricing_unit_id** | **String** | ユニットID(unit id) | [required] |
**body** | Option<**crate::models::PricingUnitForSave**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

