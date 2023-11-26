# \AwsMarketplaceApi

All URIs are relative to *https://api.saasus.io/v1/awsmarketplace*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer**](AwsMarketplaceApi.md#create_customer) | **Post** /customers | AWS Marketplaceに連携する顧客情報を新規作成(Create customer information to be linked to AWS Marketplace)
[**get_catalog_entity_visibility**](AwsMarketplaceApi.md#get_catalog_entity_visibility) | **Get** /catalog-entity/visibility | AWS Marketplaceから商品の公開状況を取得(Obtain product publication status from AWS Marketplace)
[**get_cloud_formation_launch_stack_link**](AwsMarketplaceApi.md#get_cloud_formation_launch_stack_link) | **Get** /cloudformation-launch-stack-link | AWS CloudFormationのスタック作成リンクを取得(Get the link to create the AWS CloudFormation stack)
[**get_customer**](AwsMarketplaceApi.md#get_customer) | **Get** /customers/{customer_identifier} | AWS Marketplaceに連携する顧客情報を取得(Get customer information to be linked to AWS Marketplace)
[**get_customers**](AwsMarketplaceApi.md#get_customers) | **Get** /customers | AWS Marketplaceに連携する顧客情報の一覧を取得(Get a list of customer information to be linked to AWS Marketplace)
[**get_listing_status**](AwsMarketplaceApi.md#get_listing_status) | **Get** /listing-status | AWS Marketplaceの出品状況を取得(Get AWS Marketplace Listing Status)
[**get_plan_by_plan_name**](AwsMarketplaceApi.md#get_plan_by_plan_name) | **Get** /plans/{plan_name} | AWSMarketplaceに連携するプラン情報を取得(Obtain plan information to link to AWS Marketplace)
[**get_plans**](AwsMarketplaceApi.md#get_plans) | **Get** /plans | AWS Marketplaceに連携するプラン情報を取得(Obtain plan information to link to AWS Marketplace)
[**get_settings**](AwsMarketplaceApi.md#get_settings) | **Get** /settings | AWS Marketplaceの設定を取得(Get AWS Marketplace Settings)
[**save_plan**](AwsMarketplaceApi.md#save_plan) | **Put** /plans | AWS Marketplaceに連携するプラン情報を登録(Save plan information to be linked to AWSMarketplace)
[**sync_customer**](AwsMarketplaceApi.md#sync_customer) | **Post** /customers/{customer_identifier}/sync | AWS Marketplaceの顧客情報をSaaSusに同期します(Sync AWS Marketplace customer information to SaaSus)
[**update_listing_status**](AwsMarketplaceApi.md#update_listing_status) | **Put** /listing-status | AWS Marketplaceの出品状況を更新(Update AWS Marketplace Listing Status)
[**update_settings**](AwsMarketplaceApi.md#update_settings) | **Put** /settings | AWS Marketplaceの設定を更新(Update AWS Marketplace Settings)
[**verify_registration_token**](AwsMarketplaceApi.md#verify_registration_token) | **Post** /registration-token/verify | Registration Tokenを検証(Verify Registration Token)



## create_customer

> crate::models::Customer create_customer(create_customer_param)
AWS Marketplaceに連携する顧客情報を新規作成(Create customer information to be linked to AWS Marketplace)

AWS Marketplaceに連携する顧客情報を新規作成します。  Create customer information to be linked to AWS Marketplace. 

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
AWS Marketplaceから商品の公開状況を取得(Obtain product publication status from AWS Marketplace)

AWS Marketplaceから商品の公開状況を取得します。  Retrieve the product's publication status from AWS Marketplace. 

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
AWS CloudFormationのスタック作成リンクを取得(Get the link to create the AWS CloudFormation stack)

CloudFormationのクイック作成リンクを取得します。  Get the CloudFormation Quick Create link. 

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
AWS Marketplaceに連携する顧客情報を取得(Get customer information to be linked to AWS Marketplace)

AWS Marketplaceに連携する顧客情報を取得します。  Get customer information to be linked to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_identifier** | **String** | 顧客ID | [required] |

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
AWS Marketplaceに連携する顧客情報の一覧を取得(Get a list of customer information to be linked to AWS Marketplace)

AWS Marketplaceに連携する顧客情報の一覧を取得します。  Get a list of customer information to be linked to AWS Marketplace. 

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
AWS Marketplaceの出品状況を取得(Get AWS Marketplace Listing Status)

AWS Marketplaceの出品状況を取得します。  Get AWS Marketplace Listing Status. 

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
AWSMarketplaceに連携するプラン情報を取得(Obtain plan information to link to AWS Marketplace)

Marketplaceと連携するプラン情報を取得します。  Obtain plan information to link to AWS Marketplace. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_name** | **String** | AWS Marketplace連携プラン名 | [required] |

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
AWS Marketplaceに連携するプラン情報を取得(Obtain plan information to link to AWS Marketplace)

Marketplaceと連携するプラン情報を取得します。  Obtain plan information to link to AWS Marketplace. 

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
AWS Marketplaceの設定を取得(Get AWS Marketplace Settings)

AWS Marketplaceの設定を取得します。  Get AWS Marketplace Settings. 

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
AWS Marketplaceに連携するプラン情報を登録(Save plan information to be linked to AWSMarketplace)

AWSMarketplaceに連携するプラン情報を登録します。  Save plan information to be linked to AWSMarketplace. 

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
AWS Marketplaceの顧客情報をSaaSusに同期します(Sync AWS Marketplace customer information to SaaSus)

AWS Marketplaceの顧客情報をSaaSusに同期します。  Sync AWS Marketplace customer information to SaaSus. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_identifier** | **String** | 顧客ID | [required] |

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
AWS Marketplaceの出品状況を更新(Update AWS Marketplace Listing Status)

AWS Marketplaceの出品状況を更新します。  Update AWS Marketplace Listing Status. 

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
AWS Marketplaceの設定を更新(Update AWS Marketplace Settings)

AWS Marketplaceの設定を更新します。  Update AWS Marketplace Settings. 

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
Registration Tokenを検証(Verify Registration Token)

Registration Tokenを検証します。  Verify Registration Token. 

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

