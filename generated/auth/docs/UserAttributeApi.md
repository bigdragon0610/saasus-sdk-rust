# \UserAttributeApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saas_user_attribute**](UserAttributeApi.md#create_saas_user_attribute) | **Post** /saas-user-attributes | Create SaaS User Attributes
[**create_user_attribute**](UserAttributeApi.md#create_user_attribute) | **Post** /user-attributes | Create User Attributes
[**delete_user_attribute**](UserAttributeApi.md#delete_user_attribute) | **Delete** /user-attributes/{attribute_name} | Delete User Attribute
[**get_user_attributes**](UserAttributeApi.md#get_user_attributes) | **Get** /user-attributes | Get User Attributes



## create_saas_user_attribute

> crate::models::Attribute create_saas_user_attribute(body)
Create SaaS User Attributes

Create additional SaaS user attributes to be kept on the SaaSus Platform. You can give common values to all tenants. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::Attribute**> |  |  |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_attribute

> crate::models::Attribute create_user_attribute(body)
Create User Attributes

Create additional user attributes to be kept on the SaaSus Platform. You can give different values to each tenant. For example, you can define items associated with a user, such as user name, birthday, etc. If you don't want personal information on the SaaS Platform side, personal information can be kept on the SaaS side without user attribute definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::Attribute**> |  |  |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute

> delete_user_attribute(attribute_name)
Delete User Attribute

Delete user attributes kept on the SaaSus Platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_name** | **String** | Attribute Name | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_attributes

> crate::models::UserAttributes get_user_attributes()
Get User Attributes

Get additional attributes of the user saved in the SaaSus Platform. For example, you can define items associated with a user, such as user name, birthday, etc. If you don't want personal information on the SaaS Platform side, personal information can be kept on the SaaS side without user attribute definition. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserAttributes**](UserAttributes.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

