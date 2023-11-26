# \UserAttributeApi

All URIs are relative to *https://api.saasus.io/v1/auth*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_attribute**](UserAttributeApi.md#create_user_attribute) | **Post** /user-attributes | ユーザー属性の作成(Create User Attributes)
[**delete_user_attribute**](UserAttributeApi.md#delete_user_attribute) | **Delete** /user-attributes/{attribute_name} | ユーザー属性の削除(Delete User Attribute)
[**get_user_attributes**](UserAttributeApi.md#get_user_attributes) | **Get** /user-attributes | ユーザー属性の一覧を取得(Get User Attributes)



## create_user_attribute

> crate::models::Attribute create_user_attribute(body)
ユーザー属性の作成(Create User Attributes)

SaaSus Platform にて保持するユーザーの追加属性を登録します。 例えば、ユーザー名を持たせる、誕生日を持たせるなど、ユーザーに紐付いた項目の定義を行うことができます。 一方で、個人情報を SaaSus Platform 側に持たせたくない場合は、このユーザー属性定義を行わずに SaaS 側で個人情報を持つことを検討してください。  Create additional user attributes to be kept on the SaaSus Platform. For example, you can define items associated with a user, such as user name, birthday, etc. If you don't want personal information on the SaaS Platform side, personal information can be kept on the SaaS side without user attribute definition. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::Attribute**> |  |  |

### Return type

[**crate::models::Attribute**](Attribute.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_attribute

> delete_user_attribute(attribute_name)
ユーザー属性の削除(Delete User Attribute)

SaaSus Platform にて保持するユーザーの追加属性を削除します。  Delete user attributes kept on the SaaSus Platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_name** | **String** | 属性名(Attribute Name) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_attributes

> crate::models::UserAttributes get_user_attributes()
ユーザー属性の一覧を取得(Get User Attributes)

SaaSus Platform にて保持するユーザーの追加属性を取得します。 例えば、ユーザー名を持たせる、誕生日を持たせるなど、ユーザーに紐付いた項目の定義を行うことができます。 一方で、個人情報を SaaSus Platform 側に持たせたくない場合は、このユーザー属性定義を行わずに SaaS 側で個人情報を持つことを検討してください。  Get additional attributes of the user saved in the SaaSus Platform. For example, you can define items associated with a user, such as user name, birthday, etc. If you don't want personal information on the SaaS Platform side, personal information can be kept on the SaaS side without user attribute definition. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserAttributes**](UserAttributes.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

