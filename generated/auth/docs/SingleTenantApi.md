# \SingleTenantApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cloud_formation_launch_stack_link_for_single_tenant**](SingleTenantApi.md#get_cloud_formation_launch_stack_link_for_single_tenant) | **Get** /single-tenant/cloudformation-launch-stack-link | Get CloudFormation Stack Launch Link For Single Tenant
[**get_single_tenant_settings**](SingleTenantApi.md#get_single_tenant_settings) | **Get** /single-tenant/settings | Retrieve the settings of the single tenant.
[**update_single_tenant_settings**](SingleTenantApi.md#update_single_tenant_settings) | **Patch** /single-tenant/settings | Update configuration information for single-tenant functionality



## get_cloud_formation_launch_stack_link_for_single_tenant

> crate::models::CloudFormationLaunchStackLink get_cloud_formation_launch_stack_link_for_single_tenant()
Get CloudFormation Stack Launch Link For Single Tenant

Get the CloudFormation stack activation link for Single Tenant. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CloudFormationLaunchStackLink**](CloudFormationLaunchStackLink.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_tenant_settings

> crate::models::SingleTenantSettings get_single_tenant_settings()
Retrieve the settings of the single tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SingleTenantSettings**](SingleTenantSettings.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_single_tenant_settings

> update_single_tenant_settings(update_single_tenant_settings_param)
Update configuration information for single-tenant functionality

Updates configuration information for single-tenant functionality Returns error if single tenant feature cannot be enabled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_single_tenant_settings_param** | Option<[**UpdateSingleTenantSettingsParam**](UpdateSingleTenantSettingsParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

