# \EventBridgeApi

All URIs are relative to *https://api.saasus.io/v1/integration*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event_bridge_event**](EventBridgeApi.md#create_event_bridge_event) | **Post** /eventbridge/event | Send Events
[**create_event_bridge_test_event**](EventBridgeApi.md#create_event_bridge_test_event) | **Post** /eventbridge/test-event | Test EventBridge Connection
[**delete_event_bridge_settings**](EventBridgeApi.md#delete_event_bridge_settings) | **Delete** /eventbridge/info | Delete EventBridge Settings
[**get_event_bridge_settings**](EventBridgeApi.md#get_event_bridge_settings) | **Get** /eventbridge/info | Get EventBridge Settings
[**save_event_bridge_settings**](EventBridgeApi.md#save_event_bridge_settings) | **Put** /eventbridge/info | Update EventBridge Settings



## create_event_bridge_event

> create_event_bridge_event(create_event_bridge_event_param)
Send Events

Send events to Amazon EventBridge. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_event_bridge_event_param** | Option<[**CreateEventBridgeEventParam**](CreateEventBridgeEventParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_event_bridge_test_event

> create_event_bridge_test_event()
Test EventBridge Connection

Send events to test the connection with Amazon EventBridge. 

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


## delete_event_bridge_settings

> delete_event_bridge_settings()
Delete EventBridge Settings

Delete settings used to provide host state via Amazon EventBridge. 

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


## get_event_bridge_settings

> crate::models::EventBridgeSettings get_event_bridge_settings()
Get EventBridge Settings

Gets the settings for providing real-time status of all monitored hosts via Amazon EventBridge. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EventBridgeSettings**](EventBridgeSettings.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_event_bridge_settings

> save_event_bridge_settings(body)
Update EventBridge Settings

Update configuration used to provide the host state via Amazon EventBridge. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::EventBridgeSettings**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

