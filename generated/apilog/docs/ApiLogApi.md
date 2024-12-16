# \ApiLogApi

All URIs are relative to *https://api.saasus.io/v1/apilog*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log**](ApiLogApi.md#get_log) | **Get** /logs/{api_log_id} | Get API execution log
[**get_logs**](ApiLogApi.md#get_logs) | **Get** /logs | Get API execution log list



## get_log

> crate::models::ApiLog get_log(api_log_id)
Get API execution log

Retrieve the log of the API execution with the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_log_id** | **String** | API Log ID | [required] |

### Return type

[**crate::models::ApiLog**](ApiLog.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs

> crate::models::ApiLogs get_logs(created_date, created_at, limit, cursor)
Get API execution log list

Retrieve the log of all API executions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_date** | Option<**String**> | The date, in format of YYYY-MM-DD, to retrieve the log. |  |
**created_at** | Option<**String**> | The datetime, in ISO 8601 format, to retrieve the log. |  |
**limit** | Option<**i64**> | Maximum number of logs to retrieve. |  |
**cursor** | Option<**String**> | Cursor for cursor pagination. |  |

### Return type

[**crate::models::ApiLogs**](ApiLogs.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

