# \StripeApi

All URIs are relative to *https://api.saasus.io/v1/billing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_stripe_info**](StripeApi.md#delete_stripe_info) | **Delete** /stripe/info | Delete Stripe Connection
[**get_stripe_info**](StripeApi.md#get_stripe_info) | **Get** /stripe/info | Get Stripe Connection information
[**update_stripe_info**](StripeApi.md#update_stripe_info) | **Put** /stripe/info | Update Stripe Connection Info



## delete_stripe_info

> delete_stripe_info()
Delete Stripe Connection

Delete connection with external billing SaaS 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stripe_info

> crate::models::StripeInfo get_stripe_info()
Get Stripe Connection information

Get information on connnections with external billing SaaS. Currently possible to integrate with Stripe. Without integration, you will need to implement billing using the SaaSus SDK/API. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StripeInfo**](StripeInfo.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stripe_info

> update_stripe_info(update_stripe_info_param)
Update Stripe Connection Info

Updates information on connection with external billing SaaS. Currently possible to connect to Stripe. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_stripe_info_param** | Option<[**UpdateStripeInfoParam**](UpdateStripeInfoParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

