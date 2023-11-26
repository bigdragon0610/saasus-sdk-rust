# \UserInfoApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_info**](UserInfoApi.md#get_user_info) | **Get** /userinfo | ユーザー情報取得(Get User Info)



## get_user_info

> crate::models::UserInfo get_user_info(token)
ユーザー情報取得(Get User Info)

SaaS利用ユーザ(登録ユーザ)のIDトークンを元に、ユーザ情報を取得します。 IDトークンは、SaaSus Platform生成のログイン画面からログイン時にCallback URLに渡されます。 サーバ側でそのURLからIDトークンを取得し、このAPIを呼ぶことにより、該当ユーザの情報が取得できます。 取得した上には、所属テナントや役割(ロール)、料金プランなどが含まれているため、それを元に認可の実装を行うことが可能です。  User information is obtained based on the ID token of the SaaS user (registered user). The ID token is passed to the Callback URL during login from the SaaSus Platform generated login screen. User information can be obtained from calling this API with an ID token from the URL on the server side. Since the acquired tenant, role (role), price plan, etc. are included, it is possible to implement authorization based on it. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | IDトークン(ID Token) | [required] |

### Return type

[**crate::models::UserInfo**](UserInfo.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

