# \PricingMenusApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pricing_menu**](PricingMenusApi.md#create_pricing_menu) | **Post** /menus | Create a Pricing Feature Menu
[**delete_pricing_menu**](PricingMenusApi.md#delete_pricing_menu) | **Delete** /menus/{menu_id} | Delete Pricing Feature Menu
[**get_pricing_menu**](PricingMenusApi.md#get_pricing_menu) | **Get** /menus/{menu_id} | Get Pricing Feature Menu
[**get_pricing_menus**](PricingMenusApi.md#get_pricing_menus) | **Get** /menus | Get Pricing Feature Menus
[**update_pricing_menu**](PricingMenusApi.md#update_pricing_menu) | **Patch** /menus/{menu_id} | Update Pricing Feature Menu



## create_pricing_menu

> crate::models::PricingMenu create_pricing_menu(body)
Create a Pricing Feature Menu

Create a pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::SavePricingMenuParam**> |  |  |

### Return type

[**crate::models::PricingMenu**](PricingMenu.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pricing_menu

> delete_pricing_menu(menu_id)
Delete Pricing Feature Menu

Delete pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | Menu ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_menu

> crate::models::PricingMenu get_pricing_menu(menu_id)
Get Pricing Feature Menu

Get a pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | Menu ID | [required] |

### Return type

[**crate::models::PricingMenu**](PricingMenu.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing_menus

> crate::models::PricingMenus get_pricing_menus()
Get Pricing Feature Menus

Get the feature menu list. Multiple measurement units are grouped together and defined as one feature menu. Multiple feature menus defined here are combined into one billing plan. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PricingMenus**](PricingMenus.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pricing_menu

> update_pricing_menu(menu_id, body)
Update Pricing Feature Menu

Update pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | Menu ID | [required] |
**body** | Option<**crate::models::SavePricingMenuParam**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

