# \AwsMarketplaceApi

All URIs are relative to *https://api.saasus.io/v1/awsmarketplace*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer**](AwsMarketplaceApi.md#create_customer) | **Post** /customers | Create customer information to be linked to AWS Marketplace
[**get_catalog_entity_visibility**](AwsMarketplaceApi.md#get_catalog_entity_visibility) | **Get** /catalog-entity/visibility | Obtain product publication status from AWS Marketplace
[**get_cloud_formation_launch_stack_link**](AwsMarketplaceApi.md#get_cloud_formation_launch_stack_link) | **Get** /cloudformation-launch-stack-link | Get the link to create the AWS CloudFormation stack
[**get_customer**](AwsMarketplaceApi.md#get_customer) | **Get** /customers/{customer_identifier} | Get customer information to be linked to AWS Marketplace
[**get_customers**](AwsMarketplaceApi.md#get_customers) | **Get** /customers | Get a list of customer information to be linked to AWS Marketplace
[**get_listing_status**](AwsMarketplaceApi.md#get_listing_status) | **Get** /listing-status | Get AWS Marketplace Listing Status
[**get_plan_by_plan_name**](AwsMarketplaceApi.md#get_plan_by_plan_name) | **Get** /plans/{plan_name} | Obtain plan information to link to AWS Marketplace
[**get_plans**](AwsMarketplaceApi.md#get_plans) | **Get** /plans | Obtain plan information to link to AWS Marketplace
[**get_settings**](AwsMarketplaceApi.md#get_settings) | **Get** /settings | Get AWS Marketplace Settings
[**save_plan**](AwsMarketplaceApi.md#save_plan) | **Put** /plans | Save plan information to be linked to AWSMarketplace
[**sync_customer**](AwsMarketplaceApi.md#sync_customer) | **Post** /customers/{customer_identifier}/sync | Sync AWS Marketplace customer information to SaaSus
[**update_listing_status**](AwsMarketplaceApi.md#update_listing_status) | **Put** /listing-status | Update AWS Marketplace Listing Status
[**update_settings**](AwsMarketplaceApi.md#update_settings) | **Put** /settings | Update AWS Marketplace Settings
[**verify_registration_token**](AwsMarketplaceApi.md#verify_registration_token) | **Post** /registration-token/verify | Verify Registration Token



## create_customer

> crate::models::Customer create_customer(create_customer_param)
Create customer information to be linked to AWS Marketplace

Create customer information to be linked to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_customer_param** | Option<[**CreateCustomerParam**](CreateCustomerParam.md)> |  |  |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog_entity_visibility

> crate::models::CatalogEntityVisibility get_catalog_entity_visibility()
Obtain product publication status from AWS Marketplace

Retrieve the product's publication status from AWS Marketplace. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogEntityVisibility**](CatalogEntityVisibility.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_formation_launch_stack_link

> crate::models::CloudFormationLaunchStackLink get_cloud_formation_launch_stack_link()
Get the link to create the AWS CloudFormation stack

Get the CloudFormation Quick Create link. 

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


## get_customer

> crate::models::Customer get_customer(customer_identifier)
Get customer information to be linked to AWS Marketplace

Get customer information to be linked to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_identifier** | **String** | Customer ID | [required] |

### Return type

[**crate::models::Customer**](Customer.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customers

> crate::models::Customers get_customers(tenant_ids)
Get a list of customer information to be linked to AWS Marketplace

Get a list of customer information to be linked to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_ids** | Option<[**Vec<String>**](String.md)> | 指定したテナントIDの顧客を取得する(Get customers with the specified tenant ID) |  |

### Return type

[**crate::models::Customers**](Customers.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_status

> crate::models::GetListingStatusResult get_listing_status()
Get AWS Marketplace Listing Status

Get AWS Marketplace Listing Status. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetListingStatusResult**](GetListingStatusResult.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan_by_plan_name

> crate::models::Plan get_plan_by_plan_name(plan_name)
Obtain plan information to link to AWS Marketplace

Obtain plan information to link to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_name** | **String** | AWS Marketplace linked plan name | [required] |

### Return type

[**crate::models::Plan**](Plan.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plans

> crate::models::Plans get_plans()
Obtain plan information to link to AWS Marketplace

Obtain plan information to link to AWS Marketplace. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Plans**](Plans.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_settings

> crate::models::Settings get_settings()
Get AWS Marketplace Settings

Get AWS Marketplace Settings. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Settings**](Settings.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_plan

> save_plan(save_plan_param)
Save plan information to be linked to AWSMarketplace

Save plan information to be linked to AWSMarketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_plan_param** | Option<[**SavePlanParam**](SavePlanParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_customer

> sync_customer(customer_identifier)
Sync AWS Marketplace customer information to SaaSus

Sync AWS Marketplace customer information to SaaSus. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_identifier** | **String** | Customer ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_listing_status

> update_listing_status(update_listing_status_param)
Update AWS Marketplace Listing Status

Update AWS Marketplace Listing Status. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_listing_status_param** | Option<[**UpdateListingStatusParam**](UpdateListingStatusParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_settings

> update_settings(update_settings_param)
Update AWS Marketplace Settings

Update AWS Marketplace Settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_settings_param** | Option<[**UpdateSettingsParam**](UpdateSettingsParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_registration_token

> verify_registration_token(verify_registration_token_param)
Verify Registration Token

Verify Registration Token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_registration_token_param** | Option<[**VerifyRegistrationTokenParam**](VerifyRegistrationTokenParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

