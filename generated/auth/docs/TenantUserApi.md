# \TenantUserApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant_user**](TenantUserApi.md#create_tenant_user) | **Post** /tenants/{tenant_id}/users | Create Tenant User
[**create_tenant_user_roles**](TenantUserApi.md#create_tenant_user_roles) | **Post** /tenants/{tenant_id}/users/{user_id}/envs/{env_id}/roles | Create Tenant User Role
[**delete_tenant_user**](TenantUserApi.md#delete_tenant_user) | **Delete** /tenants/{tenant_id}/users/{user_id} | Delete Tenant User
[**delete_tenant_user_role**](TenantUserApi.md#delete_tenant_user_role) | **Delete** /tenants/{tenant_id}/users/{user_id}/envs/{env_id}/roles/{role_name} | Remove Role From Tenant User
[**get_all_tenant_user**](TenantUserApi.md#get_all_tenant_user) | **Get** /tenants/all/users/{user_id} | Get User Info
[**get_all_tenant_users**](TenantUserApi.md#get_all_tenant_users) | **Get** /tenants/all/users | Get Users
[**get_tenant_user**](TenantUserApi.md#get_tenant_user) | **Get** /tenants/{tenant_id}/users/{user_id} | Get Tenant User
[**get_tenant_users**](TenantUserApi.md#get_tenant_users) | **Get** /tenants/{tenant_id}/users | Get Tenant Users
[**update_tenant_user**](TenantUserApi.md#update_tenant_user) | **Patch** /tenants/{tenant_id}/users/{user_id} | Update Tenant User Attribute



## create_tenant_user

> crate::models::User create_tenant_user(tenant_id, create_tenant_user_param)
Create Tenant User

Create a tenant user. If attributes is empty, the additional attributes will be created empty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
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
Create Tenant User Role

Create roles on tenant users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**user_id** | **String** | User ID | [required] |
**env_id** | **i32** | Env ID | [required] |
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
Delete Tenant User

Delete a user from the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**user_id** | **String** | User ID | [required] |

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
Remove Role From Tenant User

Remove a role from a tenant user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**user_id** | **String** | User ID | [required] |
**env_id** | **i32** | Env ID | [required] |
**role_name** | **String** | Role name | [required] |

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
Get User Info

Get information on user belonging to the tenant from the user ID. If the user belongs to multiple tenants, it will be returned as another object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

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
Get Users

Get all users belonging to the tenant. The same user belonging to multiple tenants will be returned as a different object. Id is not unique. 

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
Get Tenant User

Get one tenant user by specific ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**user_id** | **String** | User ID | [required] |

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
Get Tenant Users

Get all the users belonging to the tenant. Id is unique. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

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
Update Tenant User Attribute

Update tenant user attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**user_id** | **String** | User ID | [required] |
**update_tenant_user_param** | Option<[**UpdateTenantUserParam**](UpdateTenantUserParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

