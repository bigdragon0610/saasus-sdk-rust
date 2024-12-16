# \InvitationApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant_invitation**](InvitationApi.md#create_tenant_invitation) | **Post** /tenants/{tenant_id}/invitations | Create Tenant Invitation
[**delete_tenant_invitation**](InvitationApi.md#delete_tenant_invitation) | **Delete** /tenants/{tenant_id}/invitations/{invitation_id} | Delete Tenant Invitation
[**get_invitation_validity**](InvitationApi.md#get_invitation_validity) | **Get** /invitations/{invitation_id}/validity | Get Invitation Validity
[**get_tenant_invitation**](InvitationApi.md#get_tenant_invitation) | **Get** /tenants/{tenant_id}/invitations/{invitation_id} | Get Tenant Invitation
[**get_tenant_invitations**](InvitationApi.md#get_tenant_invitations) | **Get** /tenants/{tenant_id}/invitations | Get Tenant Invitations
[**validate_invitation**](InvitationApi.md#validate_invitation) | **Patch** /invitations/{invitation_id}/validate | Validate Invitation



## create_tenant_invitation

> crate::models::Invitation create_tenant_invitation(tenant_id, create_tenant_invitation_param)
Create Tenant Invitation

Create an invitation to the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**create_tenant_invitation_param** | Option<[**CreateTenantInvitationParam**](CreateTenantInvitationParam.md)> |  |  |

### Return type

[**crate::models::Invitation**](Invitation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_invitation

> delete_tenant_invitation(tenant_id, invitation_id)
Delete Tenant Invitation

Delete an invitation for the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**invitation_id** | **String** | Invitation ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invitation_validity

> crate::models::InvitationValidity get_invitation_validity(invitation_id)
Get Invitation Validity

Get the validity of an invitation to the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | Invitation ID | [required] |

### Return type

[**crate::models::InvitationValidity**](InvitationValidity.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_invitation

> crate::models::Invitation get_tenant_invitation(tenant_id, invitation_id)
Get Tenant Invitation

Get invitation information for the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |
**invitation_id** | **String** | Invitation ID | [required] |

### Return type

[**crate::models::Invitation**](Invitation.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_invitations

> crate::models::Invitations get_tenant_invitations(tenant_id)
Get Tenant Invitations

Get a list of invitations to the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Tenant ID | [required] |

### Return type

[**crate::models::Invitations**](Invitations.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_invitation

> validate_invitation(invitation_id, validate_invitation_param)
Validate Invitation

Validate an invitation to the tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | Invitation ID | [required] |
**validate_invitation_param** | Option<[**ValidateInvitationParam**](ValidateInvitationParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

