# \TaxRateApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tax_rate**](TaxRateApi.md#create_tax_rate) | **Post** /tax-rates | Create Tax Rate
[**get_tax_rates**](TaxRateApi.md#get_tax_rates) | **Get** /tax-rates | Get Tax Rates
[**update_tax_rate**](TaxRateApi.md#update_tax_rate) | **Patch** /tax-rates/{tax_rate_id} | Update Tax Rate



## create_tax_rate

> crate::models::TaxRate create_tax_rate(body)
Create Tax Rate

Creates a tax rate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::TaxRateProps**> |  |  |

### Return type

[**crate::models::TaxRate**](TaxRate.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tax_rates

> crate::models::TaxRates get_tax_rates()
Get Tax Rates

Get all Tax Rates 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TaxRates**](TaxRates.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tax_rate

> update_tax_rate(tax_rate_id, update_tax_rate_param)
Update Tax Rate

Update tax rate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tax_rate_id** | **String** | Tax Rate ID | [required] |
**update_tax_rate_param** | Option<[**UpdateTaxRateParam**](UpdateTaxRateParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

