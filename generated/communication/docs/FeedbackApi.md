# \FeedbackApi

All URIs are relative to *https://api.saasus.io/v1/communication*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feedback**](FeedbackApi.md#create_feedback) | **Post** /feedbacks | 
[**create_feedback_comment**](FeedbackApi.md#create_feedback_comment) | **Post** /feedbacks/{feedback_id}/comments | 
[**create_vote_user**](FeedbackApi.md#create_vote_user) | **Post** /feedbacks/{feedback_id}/votes/users | 
[**delete_feedback**](FeedbackApi.md#delete_feedback) | **Delete** /feedbacks/{feedback_id} | 
[**delete_feedback_comment**](FeedbackApi.md#delete_feedback_comment) | **Delete** /feedbacks/{feedback_id}/comments/{comment_id} | 
[**delete_vote_for_feedback**](FeedbackApi.md#delete_vote_for_feedback) | **Delete** /feedbacks/{feedback_id}/votes/users/{user_id} | 
[**get_feedback**](FeedbackApi.md#get_feedback) | **Get** /feedbacks/{feedback_id} | 
[**get_feedback_comment**](FeedbackApi.md#get_feedback_comment) | **Get** /feedbacks/{feedback_id}/comments/{comment_id} | 
[**get_feedbacks**](FeedbackApi.md#get_feedbacks) | **Get** /feedbacks | 
[**update_feedback**](FeedbackApi.md#update_feedback) | **Patch** /feedbacks/{feedback_id} | 
[**update_feedback_comment**](FeedbackApi.md#update_feedback_comment) | **Patch** /feedbacks/{feedback_id}/comments/{comment_id} | 
[**update_feedback_status**](FeedbackApi.md#update_feedback_status) | **Patch** /feedbacks/{feedback_id}/status | 



## create_feedback

> crate::models::Feedback create_feedback(create_feedback_param)


フィードバックを起票

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_feedback_param** | Option<[**CreateFeedbackParam**](CreateFeedbackParam.md)> |  |  |

### Return type

[**crate::models::Feedback**](Feedback.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feedback_comment

> crate::models::Comment create_feedback_comment(feedback_id, create_feedback_comment_param)


フィードバックへのコメント

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**create_feedback_comment_param** | Option<[**CreateFeedbackCommentParam**](CreateFeedbackCommentParam.md)> |  |  |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vote_user

> crate::models::Votes create_vote_user(feedback_id, create_vote_user_param)


フィードバックへの投票

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**create_vote_user_param** | Option<[**CreateVoteUserParam**](CreateVoteUserParam.md)> |  |  |

### Return type

[**crate::models::Votes**](Votes.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feedback

> delete_feedback(feedback_id)


フィードバックの削除

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feedback_comment

> delete_feedback_comment(feedback_id, comment_id)


フィードバックへのコメント削除

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vote_for_feedback

> delete_vote_for_feedback(feedback_id, user_id)


フィードバックへの投票の取消

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feedback

> crate::models::Feedback get_feedback(feedback_id)


フィードバックの取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |

### Return type

[**crate::models::Feedback**](Feedback.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feedback_comment

> crate::models::Comment get_feedback_comment(feedback_id, comment_id)


フィードバックへのコメント取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |

### Return type

[**crate::models::Comment**](Comment.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feedbacks

> crate::models::Feedbacks get_feedbacks()


フィードバックの一覧を取得

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Feedbacks**](Feedbacks.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feedback

> update_feedback(feedback_id, update_feedback_param)


フィードバックの編集

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**update_feedback_param** | Option<[**UpdateFeedbackParam**](UpdateFeedbackParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feedback_comment

> update_feedback_comment(feedback_id, comment_id, update_feedback_comment_param)


フィードバックへのコメント編集

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**update_feedback_comment_param** | Option<[**UpdateFeedbackCommentParam**](UpdateFeedbackCommentParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feedback_status

> update_feedback_status(feedback_id, update_feedback_status_param)


フィードバックのステータス更新

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feedback_id** | **String** |  | [required] |
**update_feedback_status_param** | Option<[**UpdateFeedbackStatusParam**](UpdateFeedbackStatusParam.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

