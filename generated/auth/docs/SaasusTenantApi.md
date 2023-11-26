# \SaasusTenantApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](SaasusTenantApi.md#create_api_key) | **Post** /apikeys | APIキーを作成(Create API Key)
[**delete_api_key**](SaasusTenantApi.md#delete_api_key) | **Delete** /apikeys/{api_key} | APIキーを削除(Delete API Key)
[**get_api_keys**](SaasusTenantApi.md#get_api_keys) | **Get** /apikeys | APIキー一覧を取得(Get API Keys)
[**get_client_secret**](SaasusTenantApi.md#get_client_secret) | **Get** /client-secret | クライアントシークレットを取得(Get Client Secret)
[**get_saas_id**](SaasusTenantApi.md#get_saas_id) | **Get** /saasid | SaasIDを取得(Get SaasID)
[**update_client_secret**](SaasusTenantApi.md#update_client_secret) | **Patch** /client-secret | クライアントシークレットを更新(Update Client Secret)
[**update_saas_id**](SaasusTenantApi.md#update_saas_id) | **Patch** /saasid | SaasIDを更新(Update SaasID)



## create_api_key

> create_api_key()
APIキーを作成(Create API Key)

サーバサイド用に API キーを発行します。 最大 2 つまで発行できます。  Generate an API key for the server side. Up to 2 can be generated. 

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


## delete_api_key

> delete_api_key(api_key)
APIキーを削除(Delete API Key)

サーバサイド用の API キーを削除します。  Delete API Key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | APIキー(API key) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> crate::models::ApiKeys get_api_keys()
APIキー一覧を取得(Get API Keys)

サーバサイド用に API キーを取得します。 最大 2 つまで発行できます。  Get API key for the server side. Up to 2 can be generated. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiKeys**](ApiKeys.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_secret

> crate::models::ClientSecret get_client_secret()
クライアントシークレットを取得(Get Client Secret)

API リクエストでアプリが使用する固定文字列を取得します。  Gets the fixed string that the app uses in API requests. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClientSecret**](ClientSecret.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saas_id

> crate::models::SaasId get_saas_id()
SaasIDを取得(Get SaasID)

テナントのSaasIDを取得します。 SaaSus API および SaaSus SDK にて利用します。  Get the tenant's SaasID. Used by SaaSus API and SaaSus SDK. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SaasId**](SaasId.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client_secret

> update_client_secret()
クライアントシークレットを更新(Update Client Secret)

API リクエストでアプリが使用する固定文字列を再発行します。 既に稼働中のSaaSアプリケーションに設定している場合には、動作に影響があります。  Reissue fixed strings that apps use in API requests. If changed on a SaaS application that is already running, it will affect the behavior. 

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


## update_saas_id

> update_saas_id()
SaasIDを更新(Update SaasID)

テナントのSaasIDを更新します。 SaaSus API および SaaSus SDK にて利用します。 既に稼働中の SaaS アプリケーションに設定している場合には、動作に影響があります。  Update the tenant's SaasID. Used by SaaSus API and SaaSus SDK. If changed on an SaaS application that is already running, it will affect the behavior. 

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

