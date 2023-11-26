# \TenantUserApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant_user**](TenantUserApi.md#create_tenant_user) | **Post** /tenants/{tenant_id}/users | テナントにユーザーを作成(Create Tenant User)
[**create_tenant_user_roles**](TenantUserApi.md#create_tenant_user_roles) | **Post** /tenants/{tenant_id}/users/{user_id}/envs/{env_id}/roles | テナントのユーザー情報に役割(ロール)を作成(Create Tenant User Role)
[**delete_tenant_user**](TenantUserApi.md#delete_tenant_user) | **Delete** /tenants/{tenant_id}/users/{user_id} | テナントのユーザー情報を削除(Delete Tenant User)
[**delete_tenant_user_role**](TenantUserApi.md#delete_tenant_user_role) | **Delete** /tenants/{tenant_id}/users/{user_id}/envs/{env_id}/roles/{role_name} | テナントのユーザーから役割(ロール)を削除(Remove Role From Tenant User)
[**get_all_tenant_user**](TenantUserApi.md#get_all_tenant_user) | **Get** /tenants/all/users/{user_id} | ユーザー情報を取得(Get User Info)
[**get_all_tenant_users**](TenantUserApi.md#get_all_tenant_users) | **Get** /tenants/all/users | ユーザー一覧を取得(Get Users)
[**get_tenant_user**](TenantUserApi.md#get_tenant_user) | **Get** /tenants/{tenant_id}/users/{user_id} | テナントのユーザー情報を取得(Get Tenant User)
[**get_tenant_users**](TenantUserApi.md#get_tenant_users) | **Get** /tenants/{tenant_id}/users | テナントのユーザー一覧を取得(Get Tenant Users)
[**update_tenant_user**](TenantUserApi.md#update_tenant_user) | **Patch** /tenants/{tenant_id}/users/{user_id} | テナントのユーザー属性情報を更新(Update Tenant User Attribute)



## create_tenant_user

> crate::models::User create_tenant_user(tenant_id, create_tenant_user_param)
テナントにユーザーを作成(Create Tenant User)

テナントにユーザーを作成します。 attributesを空のオブジェクトにした場合、追加属性は空で作成されます。  Create a tenant user. If attributes is empty, the additional attributes will be created empty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**create_tenant_user_param** | Option<[**CreateTenantUserParam**](CreateTenantUserParam.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant_user_roles

> create_tenant_user_roles(tenant_id, user_id, env_id, create_tenant_user_roles_param)
テナントのユーザー情報に役割(ロール)を作成(Create Tenant User Role)

テナントのユーザーに役割(ロール)を作成します。  Create roles on tenant users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |
**env_id** | **i32** | 環境ID(Env ID) | [required] |
**create_tenant_user_roles_param** | Option<[**CreateTenantUserRolesParam**](CreateTenantUserRolesParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_user

> delete_tenant_user(tenant_id, user_id)
テナントのユーザー情報を削除(Delete Tenant User)

テナントからユーザーを削除します。  Delete a user from your tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_user_role

> delete_tenant_user_role(tenant_id, user_id, env_id, role_name)
テナントのユーザーから役割(ロール)を削除(Remove Role From Tenant User)

テナントのユーザーから役割(ロール)を削除します。  Remove a role from a tenant user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |
**env_id** | **i32** | 環境ID(Env ID) | [required] |
**role_name** | **String** | 役割(ロール)名(role name) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tenant_user

> crate::models::Users get_all_tenant_user(user_id)
ユーザー情報を取得(Get User Info)

ユーザーIDからテナントに所属しているユーザー情報を取得します。 複数テナントに所属している場合は別のオブジェクトとして返却されます。  Get information on user belonging to the tenant from the user ID. If the user belongs to multiple tenants, it will be returned as another object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |

### Return type

[**crate::models::Users**](Users.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tenant_users

> crate::models::Users get_all_tenant_users()
ユーザー一覧を取得(Get Users)

テナントに所属しているユーザー全件を取得します。 複数テナントに所属する同一ユーザーは別のオブジェクトとして返却されます。 idは一意ではありません。  Get all users belonging to the tenant. The same user belonging to multiple tenants will be returned as a different object. Id is not unique. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Users**](Users.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_user

> crate::models::User get_tenant_user(tenant_id, user_id)
テナントのユーザー情報を取得(Get Tenant User)

テナントのユーザーをIDから一件取得します。  Get one tenant user by specific ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_users

> crate::models::Users get_tenant_users(tenant_id)
テナントのユーザー一覧を取得(Get Tenant Users)

テナントに所属するユーザーを全件取得します。 idは一意です。  Get all the users belonging to the tenant. Id is unique. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |

### Return type

[**crate::models::Users**](Users.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_user

> update_tenant_user(tenant_id, user_id, update_tenant_user_param)
テナントのユーザー属性情報を更新(Update Tenant User Attribute)

テナントのユーザー属性情報を更新します。  Update tenant user attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |
**update_tenant_user_param** | Option<[**UpdateTenantUserParam**](UpdateTenantUserParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

