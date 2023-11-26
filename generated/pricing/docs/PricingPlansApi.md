# \PricingPlansApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pricing_plan**](PricingPlansApi.md#create_pricing_plan) | **Post** /plans | 料金プランを作成(Create Pricing Plan)
[**delete_all_plans_and_menus_and_units_and_meters_and_tax_rates**](PricingPlansApi.md#delete_all_plans_and_menus_and_units_and_meters_and_tax_rates) | **Delete** /plans-initialization | 全てのPlans,Menus,Units,Metersの削除(Delete all Plans, Menus, Units, Meters and Tax Rates)
[**delete_pricing_plan**](PricingPlansApi.md#delete_pricing_plan) | **Delete** /plans/{plan_id} | 料金プランを削除(Delete Pricing Plan)
[**delete_stripe_plan**](PricingPlansApi.md#delete_stripe_plan) | **Delete** /stripe | stripe上の商品情報を削除(Delete Product Data from Stripe)
[**get_pricing_plan**](PricingPlansApi.md#get_pricing_plan) | **Get** /plans/{plan_id} | 料金プランを取得(Get Pricing Plan)
[**get_pricing_plans**](PricingPlansApi.md#get_pricing_plans) | **Get** /plans | 料金プラン一覧を取得(Get pricing plan list)
[**link_plan_to_stripe**](PricingPlansApi.md#link_plan_to_stripe) | **Patch** /stripe/init | stripe連携(Connect to Stripe)
[**update_pricing_plan**](PricingPlansApi.md#update_pricing_plan) | **Patch** /plans/{plan_id} | 料金プランを更新(Update Pricing Plan)
[**update_pricing_plans_used**](PricingPlansApi.md#update_pricing_plans_used) | **Patch** /plans/used | 使用済みフラグ更新(Update Used Flag)



## create_pricing_plan

> crate::models::PricingPlan create_pricing_plan(body)
料金プランを作成(Create Pricing Plan)

料金プランを作成します。  Create pricing plan. 

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
全てのPlans,Menus,Units,Metersの削除(Delete all Plans, Menus, Units, Meters and Tax Rates)

無条件に全料金プラン、メニュー、ユニット、メーター、税率を削除します。  Unconditionally remove all rate plans, menus, units, meters and tax rates. 

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
料金プランを削除(Delete Pricing Plan)

料金プランを削除します。  Delete pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | 料金プランID(price plan ID) | [required] |

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
stripe上の商品情報を削除(Delete Product Data from Stripe)

stripe上の商品情報を削除します。  Delete product data from Stripe. 

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
料金プランを取得(Get Pricing Plan)

料金プランを取得します。  Get pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | 料金プランID(price plan ID) | [required] |

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
料金プラン一覧を取得(Get pricing plan list)

料金プラン一覧を取得します。 機能メニューを複数まとめて、１つの料金プランとして定義します。 ここで定義した料金プランを各テナントは選ぶことができます。 もし特定テナント特有の料金（プライベートプライシング）がある場合は、そのテナント専用の料金プランを作成して結びつけます。  Get pricing plans. Multiple feature menus are grouped together and defined as one pricing plan. Each tenant can choose a pricing plan defined here. If you have a specific tenant-specific rate (private pricing), create and connect the pricing plan specifically for that tenant. 

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
stripe連携(Connect to Stripe)

stripeへ情報を連携します。  Connect information to Stripe. 

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
料金プランを更新(Update Pricing Plan)

料金プランを更新します。  Update pricing plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | 料金プランID(price plan ID) | [required] |
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
使用済みフラグ更新(Update Used Flag)

料金プランと配下のメニュー・ユニットを使用済みに更新します。  Update price plan and feature menu/pricing unit to used. 

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

