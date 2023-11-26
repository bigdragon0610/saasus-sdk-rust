# \EnvApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_env**](EnvApi.md#create_env) | **Post** /envs | 環境情報を作成(Create Env Info)
[**delete_env**](EnvApi.md#delete_env) | **Delete** /envs/{env_id} | 環境情報を削除(Delete Env Info)
[**get_env**](EnvApi.md#get_env) | **Get** /envs/{env_id} | 環境情報を取得(Get Env Details)
[**get_envs**](EnvApi.md#get_envs) | **Get** /envs | 環境情報一覧を取得(Get Env Info)
[**update_env**](EnvApi.md#update_env) | **Patch** /envs/{env_id} | 環境情報を更新(Update Env Info)



## create_env

> crate::models::Env create_env(body)
環境情報を作成(Create Env Info)

環境情報を作成します。 連携のテストや開発用環境や実際の運用で利用する環境など複数の環境を定義することができます。  Create environment information. Multiple environments can be defined, such as an environment for testing linkage, an environment for development, and an environment for actual operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::Env**> |  |  |

### Return type

[**crate::models::Env**](Env.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_env

> delete_env(env_id)
環境情報を削除(Delete Env Info)

環境情報を削除します。idが3の環境は削除できません。  Delete env info. Env with id 3 cannot be deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_id** | **i32** | 環境ID(Env ID) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_env

> crate::models::Env get_env(env_id)
環境情報を取得(Get Env Details)

環境情報の詳細を取得します。  Get environment details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_id** | **i32** | 環境ID(Env ID) | [required] |

### Return type

[**crate::models::Env**](Env.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_envs

> crate::models::Envs get_envs()
環境情報一覧を取得(Get Env Info)

登録されている環境情報を取得します。 連携のテストや開発用環境や実際の運用で利用する環境など複数の環境を定義することができます。  Get registered environment information. Multiple environments can be defined, such as an environment for testing linkage, an environment for development, and an environment for actual operation. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Envs**](Envs.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_env

> update_env(env_id, update_env_param)
環境情報を更新(Update Env Info)

環境情報を更新します。  Update env info. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env_id** | **i32** | 環境ID(Env ID) | [required] |
**update_env_param** | Option<[**UpdateEnvParam**](UpdateEnvParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

