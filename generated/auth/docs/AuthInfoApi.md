# \AuthInfoApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_auth_info**](AuthInfoApi.md#get_auth_info) | **Get** /auth-info | Get Authentication Info
[**get_identity_providers**](AuthInfoApi.md#get_identity_providers) | **Get** /identity-providers | Get Sign-In Information Via External Provider
[**get_sign_in_settings**](AuthInfoApi.md#get_sign_in_settings) | **Get** /sign-in-settings | Get Password Requirements
[**update_auth_info**](AuthInfoApi.md#update_auth_info) | **Put** /auth-info | Update Authentication Info
[**update_identity_provider**](AuthInfoApi.md#update_identity_provider) | **Put** /identity-providers | Update Sign-In Information
[**update_sign_in_settings**](AuthInfoApi.md#update_sign_in_settings) | **Put** /sign-in-settings | Update Password Requirements



## get_auth_info

> crate::models::AuthInfo get_auth_info()
Get Authentication Info

Get the post-login SaaS URL that contains authentication information. You can pass authentication information to the URL obtained here and implement this Callback using the SaaSus SDK. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthInfo**](AuthInfo.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_identity_providers

> crate::models::IdentityProviders get_identity_providers()
Get Sign-In Information Via External Provider

Get sign-in information via external provider set in cognito. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IdentityProviders**](IdentityProviders.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sign_in_settings

> crate::models::SignInSettings get_sign_in_settings()
Get Password Requirements

Get user password requirements. Set a secure password that is difficult to decipher by increasing the number of digits by combining alphabets, numbers, and symbols. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SignInSettings**](SignInSettings.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_auth_info

> update_auth_info(body)
Update Authentication Info

Register post-login SaaS URL for authentication information. It is possible to pass authentication information to the URL registered here and implement this Callback using the SaaSus SDK. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::AuthInfo**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_identity_provider

> update_identity_provider(update_identity_provider_param)
Update Sign-In Information

Update the sign-in information for the external ID provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_identity_provider_param** | Option<[**UpdateIdentityProviderParam**](UpdateIdentityProviderParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sign_in_settings

> update_sign_in_settings(update_sign_in_settings_param)
Update Password Requirements

Update user password requirements. Set a secure password that is difficult to decipher by increasing the number of digits by combining alphabets, numbers, and symbols. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_sign_in_settings_param** | Option<[**UpdateSignInSettingsParam**](UpdateSignInSettingsParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

