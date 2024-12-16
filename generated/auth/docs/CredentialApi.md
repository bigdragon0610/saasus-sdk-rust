# \CredentialApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auth_credentials**](CredentialApi.md#create_auth_credentials) | **Post** /credentials | Save Authentication/Authorization Information
[**get_auth_credentials**](CredentialApi.md#get_auth_credentials) | **Get** /credentials | Get Authentication/Authorization Information



## create_auth_credentials

> crate::models::AuthorizationTempCode create_auth_credentials(body)
Save Authentication/Authorization Information

Temporarily save the parameter for the ID token, access token, and refresh token and return a temporary code for obtaining. Temporary codes are valid for 10 seconds from issuance. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::Credentials**> |  |  |

### Return type

[**crate::models::AuthorizationTempCode**](AuthorizationTempCode.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_credentials

> crate::models::Credentials get_auth_credentials(code, auth_flow, refresh_token)
Get Authentication/Authorization Information

Get ID token, access token, and refresh token using a temporary code or a refresh token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> | Temp Code |  |
**auth_flow** | Option<**String**> | Authentication Flow tempCodeAuth: Getting authentication information using a temporary code refreshTokenAuth: Getting authentication information using a refresh token If not specified, it will be tempCodeAuth  |  |
**refresh_token** | Option<**String**> | Refresh Token |  |

### Return type

[**crate::models::Credentials**](Credentials.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

