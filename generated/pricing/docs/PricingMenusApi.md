# \PricingMenusApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pricing_menu**](PricingMenusApi.md#create_pricing_menu) | **Post** /menus | プライシング機能メニューを作成(Create a Pricing Feature Menu)
[**delete_pricing_menu**](PricingMenusApi.md#delete_pricing_menu) | **Delete** /menus/{menu_id} | プライシング機能メニューを削除(Delete Pricing Feature Menu)
[**get_pricing_menu**](PricingMenusApi.md#get_pricing_menu) | **Get** /menus/{menu_id} | プライシング機能メニューを取得(Get Pricing Feature Menu)
[**get_pricing_menus**](PricingMenusApi.md#get_pricing_menus) | **Get** /menus | プライシング機能メニュー一覧を取得(Get Pricing Feature Menus)
[**update_pricing_menu**](PricingMenusApi.md#update_pricing_menu) | **Patch** /menus/{menu_id} | プライシング機能メニューを更新(Updated pricing feature menu)



## create_pricing_menu

> crate::models::PricingMenu create_pricing_menu(body)
プライシング機能メニューを作成(Create a Pricing Feature Menu)

プライシング機能メニューを作成します。  Create a pricing feature menu. 

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
プライシング機能メニューを削除(Delete Pricing Feature Menu)

プライシング機能メニューを削除します。  Delete pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | メニューID(menu ID) | [required] |

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
プライシング機能メニューを取得(Get Pricing Feature Menu)

プライシング機能メニューを取得します。  Get a pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | メニューID(menu ID) | [required] |

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
プライシング機能メニュー一覧を取得(Get Pricing Feature Menus)

機能メニュー一覧を取得します。 計測単位を複数まとめて、１つの機能メニューとして定義します。 ここで定義した機能メニューを複数合わせ１つの料金プランとします。  Get the feature menu list. Multiple measurement units are grouped together and defined as one feature menu. Multiple feature menus defined here are combined into one billing plan. 

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
プライシング機能メニューを更新(Updated pricing feature menu)

プライシング機能メニューを更新します。  Update pricing feature menu. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**menu_id** | **String** | メニューID(menu ID) | [required] |
**body** | Option<**crate::models::SavePricingMenuParam**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

