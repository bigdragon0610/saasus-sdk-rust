# \SaasUserApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_email_update**](SaasUserApi.md#confirm_email_update) | **Post** /users/{user_id}/email/confirm | Confirm User Email Update
[**confirm_external_user_link**](SaasUserApi.md#confirm_external_user_link) | **Post** /external-users/confirm | Confirm External User Account Link
[**confirm_sign_up_with_aws_marketplace**](SaasUserApi.md#confirm_sign_up_with_aws_marketplace) | **Post** /aws-marketplace/sign-up-confirm | Confirm Sign Up with AWS Marketplace
[**create_saas_user**](SaasUserApi.md#create_saas_user) | **Post** /users | Create SaaS User
[**create_secret_code**](SaasUserApi.md#create_secret_code) | **Post** /users/{user_id}/mfa/software-token/secret-code | Create secret code for authentication application registration
[**delete_saas_user**](SaasUserApi.md#delete_saas_user) | **Delete** /users/{user_id} | Delete User
[**get_saas_user**](SaasUserApi.md#get_saas_user) | **Get** /users/{user_id} | Get User
[**get_saas_users**](SaasUserApi.md#get_saas_users) | **Get** /users | Get Users
[**get_user_mfa_preference**](SaasUserApi.md#get_user_mfa_preference) | **Get** /users/{user_id}/mfa/preference | Get User's MFA Settings
[**link_aws_marketplace**](SaasUserApi.md#link_aws_marketplace) | **Patch** /aws-marketplace/link | Link an existing tenant with AWS Marketplace
[**request_email_update**](SaasUserApi.md#request_email_update) | **Post** /users/{user_id}/email/request | Request User Email Update
[**request_external_user_link**](SaasUserApi.md#request_external_user_link) | **Post** /external-users/request | Request External User Account Link
[**resend_sign_up_confirmation_email**](SaasUserApi.md#resend_sign_up_confirmation_email) | **Post** /sign-up/resend | Resend Sign Up Confirmation Email
[**sign_up**](SaasUserApi.md#sign_up) | **Post** /sign-up | Sign Up
[**sign_up_with_aws_marketplace**](SaasUserApi.md#sign_up_with_aws_marketplace) | **Post** /aws-marketplace/sign-up | Sign Up with AWS Marketplace
[**unlink_provider**](SaasUserApi.md#unlink_provider) | **Delete** /users/{user_id}/providers/{provider_name} | Unlink external identity providers
[**update_saas_user_attributes**](SaasUserApi.md#update_saas_user_attributes) | **Patch** /users/{user_id}/attributes | Update SaaS User Attributes
[**update_saas_user_email**](SaasUserApi.md#update_saas_user_email) | **Patch** /users/{user_id}/email | Change Email
[**update_saas_user_password**](SaasUserApi.md#update_saas_user_password) | **Patch** /users/{user_id}/password | Change Password
[**update_software_token**](SaasUserApi.md#update_software_token) | **Put** /users/{user_id}/mfa/software-token | Register Authentication Application
[**update_user_mfa_preference**](SaasUserApi.md#update_user_mfa_preference) | **Patch** /users/{user_id}/mfa/preference | Update User's MFA Settings



## confirm_email_update

> confirm_email_update(user_id, confirm_email_update_param)
Confirm User Email Update

Verify the code to confirm the user's email address update. Requires the user's access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**confirm_email_update_param** | Option<[**ConfirmEmailUpdateParam**](ConfirmEmailUpdateParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_external_user_link

> confirm_external_user_link(confirm_external_user_link_param)
Confirm External User Account Link

Verify the code for external account user link confirmation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirm_external_user_link_param** | Option<[**ConfirmExternalUserLinkParam**](ConfirmExternalUserLinkParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_sign_up_with_aws_marketplace

> crate::models::Tenant confirm_sign_up_with_aws_marketplace(confirm_sign_up_with_aws_marketplace_param)
Confirm Sign Up with AWS Marketplace

Confirm a new use registeration linked to AWS Marketplace. Create a new tenant linked to AWS Marketplace. If the Registration Token is not valid, an error is returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirm_sign_up_with_aws_marketplace_param** | Option<[**ConfirmSignUpWithAwsMarketplaceParam**](ConfirmSignUpWithAwsMarketplaceParam.md)> |  |  |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_saas_user

> crate::models::SaasUser create_saas_user(create_saas_user_param)
Create SaaS User

Create SaaS User. If attributes is empty, a temporary password will be sent to the registered email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_saas_user_param** | Option<[**CreateSaasUserParam**](CreateSaasUserParam.md)> |  |  |

### Return type

[**crate::models::SaasUser**](SaasUser.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_secret_code

> crate::models::SoftwareTokenSecretCode create_secret_code(user_id, create_secret_code_param)
Create secret code for authentication application registration

Create a secret code for authentication application registration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**create_secret_code_param** | Option<[**CreateSecretCodeParam**](CreateSecretCodeParam.md)> |  |  |

### Return type

[**crate::models::SoftwareTokenSecretCode**](SoftwareTokenSecretCode.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saas_user

> delete_saas_user(user_id)
Delete User

Delete all users with matching user ID from the tenant and SaaS. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saas_user

> crate::models::SaasUser get_saas_user(user_id)
Get User

Get user information based on user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::SaasUser**](SaasUser.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saas_users

> crate::models::SaasUsers get_saas_users()
Get Users

Get all SaaS users. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SaasUsers**](SaasUsers.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_mfa_preference

> crate::models::MfaPreference get_user_mfa_preference(user_id)
Get User's MFA Settings

Get the user's MFA settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::MfaPreference**](MfaPreference.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_aws_marketplace

> link_aws_marketplace(link_aws_marketplace_param)
Link an existing tenant with AWS Marketplace

Link an existing tenant with AWS Marketplace. If the Registration Token is not valid, an error is returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_aws_marketplace_param** | Option<[**LinkAwsMarketplaceParam**](LinkAwsMarketplaceParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_email_update

> request_email_update(user_id, request_email_update_param)
Request User Email Update

Request to update the user's email address. Sends a verification code to the requested email address. Requires the user's access token. The verification code is valid for 24 hours. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**request_email_update_param** | Option<[**RequestEmailUpdateParam**](RequestEmailUpdateParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_external_user_link

> request_external_user_link(request_external_user_link_param)
Request External User Account Link

Request to link an external account user. Get the email address of the user to be linked from the access token and send a verification code to that email address. The verification code is valid for 24 hours. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_external_user_link_param** | Option<[**RequestExternalUserLinkParam**](RequestExternalUserLinkParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_sign_up_confirmation_email

> resend_sign_up_confirmation_email(resend_sign_up_confirmation_email_param)
Resend Sign Up Confirmation Email

Resend temporary password for the new registered user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resend_sign_up_confirmation_email_param** | Option<[**ResendSignUpConfirmationEmailParam**](ResendSignUpConfirmationEmailParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_up

> crate::models::SaasUser sign_up(sign_up_param)
Sign Up

Register a new user. A temporary password will be sent to the registered email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_param** | Option<[**SignUpParam**](SignUpParam.md)> |  |  |

### Return type

[**crate::models::SaasUser**](SaasUser.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_up_with_aws_marketplace

> crate::models::SaasUser sign_up_with_aws_marketplace(sign_up_with_aws_marketplace_param)
Sign Up with AWS Marketplace

Register a new user linked to AWS Marketplace. A temporary password will be sent to the registered email. If the Registration Token is not valid, an error is returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_with_aws_marketplace_param** | Option<[**SignUpWithAwsMarketplaceParam**](SignUpWithAwsMarketplaceParam.md)> |  |  |

### Return type

[**crate::models::SaasUser**](SaasUser.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_provider

> unlink_provider(provider_name, user_id)
Unlink external identity providers

Unlink external identity providers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saas_user_attributes

> update_saas_user_attributes(user_id, update_saas_user_attributes_param)
Update SaaS User Attributes

Update the additional attributes of the SaaS user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_saas_user_attributes_param** | Option<[**UpdateSaasUserAttributesParam**](UpdateSaasUserAttributesParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saas_user_email

> update_saas_user_email(user_id, update_saas_user_email_param)
Change Email

Change user's email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_saas_user_email_param** | Option<[**UpdateSaasUserEmailParam**](UpdateSaasUserEmailParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saas_user_password

> update_saas_user_password(user_id, update_saas_user_password_param)
Change Password

Change user's login password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_saas_user_password_param** | Option<[**UpdateSaasUserPasswordParam**](UpdateSaasUserPasswordParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_software_token

> update_software_token(user_id, update_software_token_param)
Register Authentication Application

Register an authentication application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_software_token_param** | Option<[**UpdateSoftwareTokenParam**](UpdateSoftwareTokenParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_mfa_preference

> update_user_mfa_preference(user_id, body)
Update User's MFA Settings

Update user's MFA settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | Option<**crate::models::MfaPreference**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

