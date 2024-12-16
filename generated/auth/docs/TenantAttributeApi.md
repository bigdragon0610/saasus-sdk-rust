# \TenantAttributeApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant_attribute**](TenantAttributeApi.md#create_tenant_attribute) | **Post** /tenant-attributes | Create Tenant Attribute
[**delete_tenant_attribute**](TenantAttributeApi.md#delete_tenant_attribute) | **Delete** /tenant-attributes/{attribute_name} | Delete Tenant Attribute
[**get_tenant_attributes**](TenantAttributeApi.md#get_tenant_attributes) | **Get** /tenant-attributes | Get Tenant Attributes



## create_tenant_attribute

> crate::models::Attribute create_tenant_attribute(body)
Create Tenant Attribute

Register additional tenant attributes to be managed by SaaSus Platform. For example, tenant name, memo, etc., then get the attributes from SaaS using the SaaSus SDK/API. 

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


## delete_tenant_attribute

> delete_tenant_attribute(attribute_name)
Delete Tenant Attribute

Deletes tenant attributes managed by SaaSus Platform. 

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


## get_tenant_attributes

> crate::models::TenantAttributes get_tenant_attributes()
Get Tenant Attributes

Get definitions for additional tenant attributes managed by the SaaSus Platform. For example, tenant name, memo, etc., then get the attributes from SaaS using the SaaSus SDK/API. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TenantAttributes**](TenantAttributes.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

