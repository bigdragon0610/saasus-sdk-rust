# \SaasUserApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_email_update**](SaasUserApi.md#confirm_email_update) | **Post** /users/{user_id}/email/confirm | ユーザーのメールアドレス変更確認(Confirm User Email Update)
[**confirm_external_user_link**](SaasUserApi.md#confirm_external_user_link) | **Post** /external-users/confirm | 外部アカウントのユーザーの連携確認(Confirm External User Account Link)
[**confirm_sign_up_with_aws_marketplace**](SaasUserApi.md#confirm_sign_up_with_aws_marketplace) | **Post** /aws-marketplace/sign-up-confirm | AWS Marketplaceによるユーザー新規登録の確定(Confirm Sign Up with AWS Marketplace)
[**create_saas_user**](SaasUserApi.md#create_saas_user) | **Post** /users | SaaSにユーザーを作成(Create SaaS User)
[**create_secret_code**](SaasUserApi.md#create_secret_code) | **Post** /users/{user_id}/mfa/software-token/secret-code | 認証アプリケーション登録用のシークレットコードを作成(Creates secret code for authentication application registration)
[**delete_saas_user**](SaasUserApi.md#delete_saas_user) | **Delete** /users/{user_id} | ユーザー情報を削除(Delete User)
[**get_saas_user**](SaasUserApi.md#get_saas_user) | **Get** /users/{user_id} | ユーザー情報を取得(Get User)
[**get_saas_users**](SaasUserApi.md#get_saas_users) | **Get** /users | ユーザー一覧を取得(Get Users)
[**get_user_mfa_preference**](SaasUserApi.md#get_user_mfa_preference) | **Get** /users/{user_id}/mfa/preference | ユーザーのMFA設定を取得(Get User's MFA Settings)
[**link_aws_marketplace**](SaasUserApi.md#link_aws_marketplace) | **Patch** /aws-marketplace/link | AWS Marketplaceと既存のテナントの連携(Link an existing tenant with AWS Marketplace)
[**request_email_update**](SaasUserApi.md#request_email_update) | **Post** /users/{user_id}/email/request | ユーザーのメールアドレス変更要求(Request User Email Update)
[**request_external_user_link**](SaasUserApi.md#request_external_user_link) | **Post** /external-users/request | 外部アカウントのユーザー連携要求(Request External User Account Link)
[**resend_sign_up_confirmation_email**](SaasUserApi.md#resend_sign_up_confirmation_email) | **Post** /sign-up/resend | 新規登録時の確認メール再送信(Resend Sign Up Confirmation Email)
[**sign_up**](SaasUserApi.md#sign_up) | **Post** /sign-up | 新規登録(Sign Up)
[**sign_up_with_aws_marketplace**](SaasUserApi.md#sign_up_with_aws_marketplace) | **Post** /aws-marketplace/sign-up | AWS Marketplaceによるユーザー新規登録(Sign Up with AWS Marketplace)
[**unlink_provider**](SaasUserApi.md#unlink_provider) | **Delete** /users/{user_id}/providers/{provider_name} | 外部IDプロバイダの連携解除(Unlink external identity providers)
[**update_saas_user_email**](SaasUserApi.md#update_saas_user_email) | **Patch** /users/{user_id}/email | メールアドレスを変更(Change Email)
[**update_saas_user_password**](SaasUserApi.md#update_saas_user_password) | **Patch** /users/{user_id}/password | パスワードを変更(Change Password)
[**update_software_token**](SaasUserApi.md#update_software_token) | **Put** /users/{user_id}/mfa/software-token | 認証アプリケーションを登録(Register Authentication Application)
[**update_user_mfa_preference**](SaasUserApi.md#update_user_mfa_preference) | **Patch** /users/{user_id}/mfa/preference | ユーザーのMFA設定を更新(Update User's MFA Settings)



## confirm_email_update

> confirm_email_update(user_id, confirm_email_update_param)
ユーザーのメールアドレス変更確認(Confirm User Email Update)

ユーザーのメールアドレス変更確認のためにコードを検証します。 ユーザーのアクセストークンが必要です。  Verify the code to confirm the user's email address update. Requires the user's access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
外部アカウントのユーザーの連携確認(Confirm External User Account Link)

外部アカウントのユーザー連携確認のためにコードを検証します。  Verify the code for external account user link confirmation. 

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
AWS Marketplaceによるユーザー新規登録の確定(Confirm Sign Up with AWS Marketplace)

AWS Marketplaceと連携したユーザー新規登録を確定します。AWS Marketplaceと連携したテナントを新規作成します。 Registration Tokenが有効でない場合はエラーを返却します。  Confirm a new use registeration linked to AWS Marketplace. Create a new tenant linked to AWS Marketplace. If the Registration Token is not valid, an error is returned. 

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
SaaSにユーザーを作成(Create SaaS User)

SaaSにユーザーを作成します。  Create SaaS User. 

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
認証アプリケーション登録用のシークレットコードを作成(Creates secret code for authentication application registration)

認証アプリケーション登録用のシークレットコードを作成します。  Create a secret code for authentication application registration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
ユーザー情報を削除(Delete User)

ユーザーIDを元に一致するユーザーをテナントからすべて削除し、SaaSからも削除します。  Delete all users with matching user ID from the tenant and SaaS. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |

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
ユーザー情報を取得(Get User)

ユーザーIDからユーザー情報を取得します。  Get user information based on user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |

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
ユーザー一覧を取得(Get Users)

SaaSのユーザー全件を取得します。  Get all SaaS users. 

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
ユーザーのMFA設定を取得(Get User's MFA Settings)

ユーザーのMFA設定を取得します。  Get the user's MFA settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |

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
AWS Marketplaceと既存のテナントの連携(Link an existing tenant with AWS Marketplace)

AWS Marketplaceと既存のテナントを連携します。 Registration Tokenが有効でない場合はエラーを返却します。  Link an existing tenant with AWS Marketplace. If the Registration Token is not valid, an error is returned. 

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
ユーザーのメールアドレス変更要求(Request User Email Update)

ユーザーのメールアドレス変更を要求します。 要求されたメールアドレスに対して検証コードを送信します。 ユーザーのアクセストークンが必要です。 検証コードの有効期限は24時間です。  Request to update the user's email address. Sends a verification code to the requested email address. Requires the user's access token. The verification code is valid for 24 hours. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
外部アカウントのユーザー連携要求(Request External User Account Link)

外部アカウントのユーザー連携を要求します。 アクセストークンから連携するユーザーのメールアドレスを取得し、そのメールアドレスに対して検証コードを送信します。 検証コードの有効期限は24時間です。  Request to link an external account user. Get the email address of the user to be linked from the access token and send a verification code to that email address. The verification code is valid for 24 hours. 

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
新規登録時の確認メール再送信(Resend Sign Up Confirmation Email)

新規登録時の仮パスワードを再送信します。  Resend temporary password for the new registered user. 

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
新規登録(Sign Up)

ユーザーを新規登録します。登録されたメールアドレスに対して仮パスワードを送信します。  Register a new user. A temporary password will be sent to the registered email. 

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
AWS Marketplaceによるユーザー新規登録(Sign Up with AWS Marketplace)

AWS Marketplaceと連携したユーザーを新規登録します。登録されたメールアドレスに対して仮パスワードを送信します。 Registration Tokenが有効でない場合はエラーを返却します。  Register a new user linked to AWS Marketplace. A temporary password will be sent to the registered email. If the Registration Token is not valid, an error is returned. 

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
外部IDプロバイダの連携解除(Unlink external identity providers)

外部IDプロバイダの連携を解除します。  Unlink external identity providers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_name** | **String** |  | [required] |
**user_id** | **String** | ユーザーID(User ID) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saas_user_email

> update_saas_user_email(user_id, update_saas_user_email_param)
メールアドレスを変更(Change Email)

ユーザーのメールアドレスを変更します。  Change user's email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
パスワードを変更(Change Password)

ユーザーのログインパスワードを変更します。  Change user's login password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
認証アプリケーションを登録(Register Authentication Application)

認証アプリケーションを登録します。  Register an authentication application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
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
ユーザーのMFA設定を更新(Update User's MFA Settings)

ユーザーのMFA設定を更新します。  Update user's MFA settings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ユーザーID(User ID) | [required] |
**body** | Option<**crate::models::MfaPreference**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

