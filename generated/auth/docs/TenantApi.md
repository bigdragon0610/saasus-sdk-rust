# \TenantApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantApi.md#create_tenant) | **Post** /tenants | テナントを作成(Create Tenant)
[**create_tenant_and_pricing**](TenantApi.md#create_tenant_and_pricing) | **Patch** /stripe/init | stripe初期設定(Stripe Initial Setting)
[**delete_stripe_tenant_and_pricing**](TenantApi.md#delete_stripe_tenant_and_pricing) | **Delete** /stripe | stripe上の顧客情報・商品情報の削除(Delete Customer and Product From Stripe)
[**delete_tenant**](TenantApi.md#delete_tenant) | **Delete** /tenants/{tenant_id} | テナント情報を削除(Delete Tenant)
[**get_tenant**](TenantApi.md#get_tenant) | **Get** /tenants/{tenant_id} | テナント情報を取得(Get Tenant Details)
[**get_tenant_identity_providers**](TenantApi.md#get_tenant_identity_providers) | **Get** /tenants/{tenant_id}/identity-providers | テナント毎の外部IDプロバイダ取得(Get identity provider per tenant)
[**get_tenants**](TenantApi.md#get_tenants) | **Get** /tenants | テナント一覧取得(Get Tenants)
[**reset_plan**](TenantApi.md#reset_plan) | **Put** /plans/reset | プランに関わる情報を全削除
[**update_tenant**](TenantApi.md#update_tenant) | **Patch** /tenants/{tenant_id} | テナント情報を更新(Update Tenant Details)
[**update_tenant_billing_info**](TenantApi.md#update_tenant_billing_info) | **Put** /tenants/{tenant_id}/billing-info | テナントの請求先情報を更新(Update Tenant Billing Information)
[**update_tenant_identity_provider**](TenantApi.md#update_tenant_identity_provider) | **Put** /tenants/{tenant_id}/identity-providers | テナント毎の外部IDプロバイダ更新(Update identity provider per tenant)
[**update_tenant_plan**](TenantApi.md#update_tenant_plan) | **Put** /tenants/{tenant_id}/plans | テナントのプラン情報を更新(Update Tenant Plan Information)



## create_tenant

> crate::models::Tenant create_tenant(body)
テナントを作成(Create Tenant)

SaaSus Platform で管理する、テナント情報を作成します。  Create a tenant managed by the SaaSus Platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::TenantProps**> |  |  |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant_and_pricing

> create_tenant_and_pricing()
stripe初期設定(Stripe Initial Setting)

billing経由でstripeへ初期情報を設定  Set Stripe initial information via billing 

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


## delete_stripe_tenant_and_pricing

> delete_stripe_tenant_and_pricing()
stripe上の顧客情報・商品情報の削除(Delete Customer and Product From Stripe)

stripe上の顧客情報・商品情報を削除します  Delete customer and product from Stripe. 

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


## delete_tenant

> delete_tenant(tenant_id)
テナント情報を削除(Delete Tenant)

SaaSus Platform で管理する、テナントの詳細情報を削除します。  Delete SaaSus Platform tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> crate::models::TenantDetail get_tenant(tenant_id)
テナント情報を取得(Get Tenant Details)

SaaSus Platform で管理する、テナントの詳細情報を取得します。  Get the details of tenant managed on the SaaSus Platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |

### Return type

[**crate::models::TenantDetail**](TenantDetail.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_identity_providers

> crate::models::TenantIdentityProviders get_tenant_identity_providers(tenant_id)
テナント毎の外部IDプロバイダ取得(Get identity provider per tenant)

テナント毎の外部IDプロバイダ経由のサインイン情報を取得します。  Get sign-in information via external identity provider per tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |

### Return type

[**crate::models::TenantIdentityProviders**](TenantIdentityProviders.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenants

> crate::models::Tenants get_tenants()
テナント一覧取得(Get Tenants)

SaaSus Platform で管理する、テナント情報の取得を行います。  Get tenants managed by SaaSus Platform. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Tenants**](Tenants.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_plan

> reset_plan()
プランに関わる情報を全削除

料金プランに関わる情報を全削除します。 テナントに連携されたプランとプラン定義を削除します。 Stripe連携している場合、連携が解除されます。  Delete all information related to rate plans. Delete plans linked to tenants and plan definitions. If you are using the Stripe linkage, the linkage will be removed. 

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


## update_tenant

> update_tenant(tenant_id, body)
テナント情報を更新(Update Tenant Details)

SaaSus Platform で管理する、テナントの詳細情報を更新します。  Update SaaSus Platform tenant details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**body** | Option<**crate::models::TenantProps**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_billing_info

> update_tenant_billing_info(tenant_id, body)
テナントの請求先情報を更新(Update Tenant Billing Information)

SaaSus Platform で管理しているテナントの請求先情報を更新します。  Update SaaSus Platform tenant billing information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**body** | Option<**crate::models::BillingInfo**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_identity_provider

> update_tenant_identity_provider(tenant_id, update_tenant_identity_provider_param)
テナント毎の外部IDプロバイダ更新(Update identity provider per tenant)

テナント毎の外部IDプロバイダ経由のサインイン情報を更新します。  Update sign-in information via external identity provider per tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**update_tenant_identity_provider_param** | Option<[**UpdateTenantIdentityProviderParam**](UpdateTenantIdentityProviderParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_plan

> update_tenant_plan(tenant_id, body)
テナントのプラン情報を更新(Update Tenant Plan Information)

SaaSus Platform で管理しているテナントのプラン情報を更新します。  Update SaaSus Platform tenant plan information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(Tenant ID) | [required] |
**body** | Option<**crate::models::PlanReservation**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

