# \PricingPlansApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pricing_plan**](PricingPlansApi.md#create_pricing_plan) | **Post** /plans | Create Pricing Plan
[**delete_all_plans_and_menus_and_units_and_meters_and_tax_rates**](PricingPlansApi.md#delete_all_plans_and_menus_and_units_and_meters_and_tax_rates) | **Delete** /plans-initialization | Delete all Plans, Menus, Units, Meters and Tax Rates
[**delete_pricing_plan**](PricingPlansApi.md#delete_pricing_plan) | **Delete** /plans/{plan_id} | Delete Pricing Plan
[**delete_stripe_plan**](PricingPlansApi.md#delete_stripe_plan) | **Delete** /stripe | Delete Product Data from Stripe
[**get_pricing_plan**](PricingPlansApi.md#get_pricing_plan) | **Get** /plans/{plan_id} | Get Pricing Plan
[**get_pricing_plans**](PricingPlansApi.md#get_pricing_plans) | **Get** /plans | Get Pricing Plans
[**link_plan_to_stripe**](PricingPlansApi.md#link_plan_to_stripe) | **Patch** /stripe/init | Connect to Stripe
[**update_pricing_plan**](PricingPlansApi.md#update_pricing_plan) | **Patch** /plans/{plan_id} | Update Pricing Plan
[**update_pricing_plans_used**](PricingPlansApi.md#update_pricing_plans_used) | **Patch** /plans/used | Update Used Flag



## create_pricing_plan

> crate::models::PricingPlan create_pricing_plan(body)
Create Pricing Plan

Create a pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::SavePricingPlanParam**> |  |  |

### Return type

[**crate::models::PricingPlan**](PricingPlan.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_plans_and_menus_and_units_and_meters_and_tax_rates

> delete_all_plans_and_menus_and_units_and_meters_and_tax_rates()
Delete all Plans, Menus, Units, Meters and Tax Rates

Unconditionally remove all rate plans, menus, units, meters and tax rates. 

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


## delete_pricing_plan

> delete_pricing_plan(plan_id)
Delete Pricing Plan

Delete a pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | Pricing Plan ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stripe_plan

> delete_stripe_plan()
Delete Product Data from Stripe

Delete product data from Stripe. 

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


## get_pricing_plan

> crate::models::PricingPlan get_pricing_plan(plan_id)
Get Pricing Plan

Get a pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | Pricing Plan ID | [required] |

### Return type

[**crate::models::PricingPlan**](PricingPlan.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_plans

> crate::models::PricingPlans get_pricing_plans()
Get Pricing Plans

Get pricing plans. Multiple feature menus are grouped together and defined as one pricing plan. Each tenant can choose a pricing plan defined here. If you have a specific tenant-specific rate (private pricing), create and connect the pricing plan specifically for that tenant. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PricingPlans**](PricingPlans.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_plan_to_stripe

> link_plan_to_stripe()
Connect to Stripe

Connect information to Stripe. 

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


## update_pricing_plan

> update_pricing_plan(plan_id, body)
Update Pricing Plan

Update a pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | Pricing Plan ID | [required] |
**body** | Option<**crate::models::SavePricingPlanParam**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pricing_plans_used

> update_pricing_plans_used(update_pricing_plans_used_param)
Update Used Flag

Update price plan and feature menu/pricing unit to used. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_pricing_plans_used_param** | Option<[**UpdatePricingPlansUsedParam**](UpdatePricingPlansUsedParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

