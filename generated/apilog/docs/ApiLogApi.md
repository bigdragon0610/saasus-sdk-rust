# \ApiLogApi

All URIs are relative to *https://api.saasus.io/v1/apilog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log**](ApiLogApi.md#get_log) | **Get** /logs/{api_log_id} | API実行ログ取得
[**get_logs**](ApiLogApi.md#get_logs) | **Get** /logs | API実行ログ取得



## get_log

> crate::models::ApiLog get_log(api_log_id)
API実行ログ取得

指定したIDのAPI実行のログ登録を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_log_id** | **String** | APIログID(API Log ID) | [required] |

### Return type

[**crate::models::ApiLog**](ApiLog.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs

> crate::models::ApiLogs get_logs()
API実行ログ取得

全API実行のログ登録を取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiLogs**](ApiLogs.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

