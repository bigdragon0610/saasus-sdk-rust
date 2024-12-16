# \TenantApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantApi.md#create_tenant) | **Post** /tenants | Create Tenant
[**create_tenant_and_pricing**](TenantApi.md#create_tenant_and_pricing) | **Patch** /stripe/init | Stripe Initial Setting
[**delete_stripe_tenant_and_pricing**](TenantApi.md#delete_stripe_tenant_and_pricing) | **Delete** /stripe | Delete Customer and Product From Stripe
[**delete_tenant**](TenantApi.md#delete_tenant) | **Delete** /tenants/{tenant_id} | Delete Tenant
[**get_stripe_customer**](TenantApi.md#get_stripe_customer) | **Get** /tenants/{tenant_id}/stripe-customer | Get Stripe Customer
[**get_tenant**](TenantApi.md#get_tenant) | **Get** /tenants/{tenant_id} | Get Tenant Details
[**get_tenant_identity_providers**](TenantApi.md#get_tenant_identity_providers) | **Get** /tenants/{tenant_id}/identity-providers | Get identity provider per tenant
[**get_tenants**](TenantApi.md#get_tenants) | **Get** /tenants | Get Tenants
[**reset_plan**](TenantApi.md#reset_plan) | **Put** /plans/reset | Delete all information related to rate plans
[**update_tenant**](TenantApi.md#update_tenant) | **Patch** /tenants/{tenant_id} | Update Tenant Details
[**update_tenant_billing_info**](TenantApi.md#update_tenant_billing_info) | **Put** /tenants/{tenant_id}/billing-info | Update Tenant Billing Information
[**update_tenant_identity_provider**](TenantApi.md#update_tenant_identity_provider) | **Put** /tenants/{tenant_id}/identity-providers | Update identity provider per tenant
[**update_tenant_plan**](TenantApi.md#update_tenant_plan) | **Put** /tenants/{tenant_id}/plans | Update Tenant Plan Information



## create_tenant

> crate::models::Tenant create_tenant(body)
Create Tenant

Create a tenant managed by the SaaSus Platform. 

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
Stripe Initial Setting

Set Stripe initial information via billing 

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
Delete Customer and Product From Stripe

Delete customer and product from Stripe. 

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
Delete Tenant

Delete SaaSus Platform tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stripe_customer

> crate::models::StripeCustomer get_stripe_customer(tenant_id)
Get Stripe Customer

Get the Stripe Customer information associated with the tenant, including their subscriptions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

### Return type

[**crate::models::StripeCustomer**](StripeCustomer.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> crate::models::TenantDetail get_tenant(tenant_id)
Get Tenant Details

Get the details of tenant managed on the SaaSus Platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

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
Get identity provider per tenant

Get sign-in information via external identity provider per tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

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
Get Tenants

Get tenants managed by SaaSus Platform. 

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
Delete all information related to rate plans

Delete all information related to rate plans. Delete plans linked to tenants and plan definitions. If you are using the Stripe linkage, the linkage will be removed. 

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
Update Tenant Details

Update SaaSus Platform tenant details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
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
Update Tenant Billing Information

Update SaaSus Platform tenant billing information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
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
Update identity provider per tenant

Update sign-in information via external identity provider per tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
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
Update Tenant Plan Information

Update SaaSus Platform tenant plan information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**body** | Option<**crate::models::PlanReservation**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

