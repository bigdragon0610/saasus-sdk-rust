# \CredentialApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auth_credentials**](CredentialApi.md#create_auth_credentials) | **Post** /credentials | 認証・認可情報の保存(Save Authentication/Authorization Information)
[**get_auth_credentials**](CredentialApi.md#get_auth_credentials) | **Get** /credentials | 認証・認可情報の取得(Get Authentication/Authorization Information)



## create_auth_credentials

> crate::models::AuthorizationTempCode create_auth_credentials(body)
認証・認可情報の保存(Save Authentication/Authorization Information)

引数のIDトークン・アクセストークン・リフレッシュトークンを一時保存し取得用の一時コードを返却する。 一時コードの有効期間は発行から10秒です。  Temporarily save the parameter for the ID token, access token, and refresh token and return a temporary code for obtaining. Temporary codes are valid for 10 seconds from issuance. 

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
認証・認可情報の取得(Get Authentication/Authorization Information)

一時コードまたはリフレッシュトークンを利用してIDトークン・アクセストークン・リフレッシュトークンを取得する。  Get ID token, access token, and refresh token using a temporary code or a refresh token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | Option<**String**> | 一時コード(Temp Code) |  |
**auth_flow** | Option<**String**> | 認証フロー（Authentication Flow） tempCodeAuth: 一時コードを利用した認証情報の取得 refreshTokenAuth: リフレッシュトークンを利用した認証情報の取得 指定されていない場合は tempCodeAuth になります  |  |
**refresh_token** | Option<**String**> | リフレッシュトークン(Refresh Token) |  |

### Return type

[**crate::models::Credentials**](Credentials.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

