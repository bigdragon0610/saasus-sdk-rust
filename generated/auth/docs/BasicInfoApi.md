# \BasicInfoApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_notification_messages**](BasicInfoApi.md#find_notification_messages) | **Get** /notification-messages | 通知メールテンプレートを取得(Get Notification Email Templates)
[**get_basic_info**](BasicInfoApi.md#get_basic_info) | **Get** /basic-info | 基本設定情報の取得(Get Basic Configurations)
[**get_customize_page_settings**](BasicInfoApi.md#get_customize_page_settings) | **Get** /customize-page-settings | 認証認可基本情報取得(Get Authentication Authorization Basic Information)
[**get_customize_pages**](BasicInfoApi.md#get_customize_pages) | **Get** /customize-pages | 認証系画面設定情報取得(Get Authentication Page Setting)
[**update_basic_info**](BasicInfoApi.md#update_basic_info) | **Put** /basic-info | 基本設定情報の更新(Update Basic Configurations)
[**update_customize_page_settings**](BasicInfoApi.md#update_customize_page_settings) | **Patch** /customize-page-settings | 認証認可基本情報更新(Update Authentication Authorization Basic Information)
[**update_customize_pages**](BasicInfoApi.md#update_customize_pages) | **Patch** /customize-pages | 認証系画面設定情報設定(Authentication Page Setting)
[**update_notification_messages**](BasicInfoApi.md#update_notification_messages) | **Put** /notification-messages | 通知メールテンプレートを更新(Update Notification Email Template)



## find_notification_messages

> crate::models::NotificationMessages find_notification_messages()
通知メールテンプレートを取得(Get Notification Email Templates)

各種通知メールテンプレートを取得します。  Get notification email templates. 

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
基本設定情報の取得(Get Basic Configurations)

SaaS ID を元に設定されているドメイン名と CNAME レコードを取得します。 取得した CNAME レコードを DNS に設定することで、ログイン画面を生成します。  Get the domain name and CNAME record based on the SaaS ID. By setting the CNAME record on the DNS the login screen will be generated. 

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
認証認可基本情報取得(Get Authentication Authorization Basic Information)

認証認可基本情報を取得します。  Get authentication authorization basic information. 

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
認証系画面設定情報取得(Get Authentication Page Setting)

認証系画面設定情報（新規登録・ログイン・パスワードリセット等）を取得します。  Get the authentication screen setting information (new registration, login, password reset, etc.). 

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
基本設定情報の更新(Update Basic Configurations)

SaaS ID を元にパラメータとして設定したドメイン名を設定更新します。 CNAME レコードが生成されますので、 DNS に設定して下さい。 既に稼働中の SaaS アプリケーションに設定している場合には、動作に影響があります。  Update the domain name that was set as a parameter based on the SaaS ID. After the CNAME record is generated, set it in your DNS. If it is set on a SaaS application that is already running, it will affect the behavior. 

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
認証認可基本情報更新(Update Authentication Authorization Basic Information)

認証認可基本情報を更新します。  Update authentication authorization basic information. 

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
認証系画面設定情報設定(Authentication Page Setting)

認証系画面設定情報（新規登録・ログイン・パスワードリセット等）を更新します。  Update the authentication page setting information (new registration, login, password reset, etc.). 

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
通知メールテンプレートを更新(Update Notification Email Template)

各種通知メールテンプレート更新します。  Update notification email template. 

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

