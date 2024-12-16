# \BasicInfoApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_notification_messages**](BasicInfoApi.md#find_notification_messages) | **Get** /notification-messages | Get Notification Email Templates
[**get_basic_info**](BasicInfoApi.md#get_basic_info) | **Get** /basic-info | Get Basic Configurations
[**get_customize_page_settings**](BasicInfoApi.md#get_customize_page_settings) | **Get** /customize-page-settings | Get Authentication Authorization Basic Information
[**get_customize_pages**](BasicInfoApi.md#get_customize_pages) | **Get** /customize-pages | Get Authentication Page Setting
[**update_basic_info**](BasicInfoApi.md#update_basic_info) | **Put** /basic-info | Update Basic Configurations
[**update_customize_page_settings**](BasicInfoApi.md#update_customize_page_settings) | **Patch** /customize-page-settings | Update Authentication Authorization Basic Information
[**update_customize_pages**](BasicInfoApi.md#update_customize_pages) | **Patch** /customize-pages | Authentication Page Setting
[**update_notification_messages**](BasicInfoApi.md#update_notification_messages) | **Put** /notification-messages | Update Notification Email Template



## find_notification_messages

> crate::models::NotificationMessages find_notification_messages()
Get Notification Email Templates

Get notification email templates. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NotificationMessages**](NotificationMessages.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_basic_info

> crate::models::BasicInfo get_basic_info()
Get Basic Configurations

Get the domain name and CNAME record based on the SaaS ID. By setting the CNAME record on the DNS the login screen will be generated. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BasicInfo**](BasicInfo.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customize_page_settings

> crate::models::CustomizePageSettings get_customize_page_settings()
Get Authentication Authorization Basic Information

Get authentication authorization basic information. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CustomizePageSettings**](CustomizePageSettings.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customize_pages

> crate::models::CustomizePages get_customize_pages()
Get Authentication Page Setting

Get the authentication screen setting information (new registration, login, password reset, etc.). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CustomizePages**](CustomizePages.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_basic_info

> update_basic_info(update_basic_info_param)
Update Basic Configurations

Update the domain name that was set as a parameter based on the SaaS ID. After the CNAME record is generated, set it in your DNS. If it is set on a SaaS application that is already running, it will affect the behavior. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_basic_info_param** | Option<[**UpdateBasicInfoParam**](UpdateBasicInfoParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_customize_page_settings

> update_customize_page_settings(update_customize_page_settings_param)
Update Authentication Authorization Basic Information

Update authentication authorization basic information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_customize_page_settings_param** | Option<[**UpdateCustomizePageSettingsParam**](UpdateCustomizePageSettingsParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_customize_pages

> update_customize_pages(update_customize_pages_param)
Authentication Page Setting

Update the authentication page setting information (new registration, login, password reset, etc.). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_customize_pages_param** | Option<[**UpdateCustomizePagesParam**](UpdateCustomizePagesParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_messages

> update_notification_messages(update_notification_messages_param)
Update Notification Email Template

Update notification email template. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_notification_messages_param** | Option<[**UpdateNotificationMessagesParam**](UpdateNotificationMessagesParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

