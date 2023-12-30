# \FeedbackApi

All URIs are relative to *https://api.saasus.io/v1/communication*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feedback**](FeedbackApi.md#create_feedback) | **Post** /feedbacks | フィードバックを起票(Create Feedback)
[**create_feedback_comment**](FeedbackApi.md#create_feedback_comment) | **Post** /feedbacks/{feedback_id}/comments | フィードバックへのコメント投稿(Create Feedback Comment)
[**create_vote_user**](FeedbackApi.md#create_vote_user) | **Post** /feedbacks/{feedback_id}/votes/users | フィードバックへの投票(Create Vote User)
[**delete_feedback**](FeedbackApi.md#delete_feedback) | **Delete** /feedbacks/{feedback_id} | フィードバックを削除(Delete Feedback)
[**delete_feedback_comment**](FeedbackApi.md#delete_feedback_comment) | **Delete** /feedbacks/{feedback_id}/comments/{comment_id} | フィードバックへのコメント削除(Delete Feedback Comment)
[**delete_vote_for_feedback**](FeedbackApi.md#delete_vote_for_feedback) | **Delete** /feedbacks/{feedback_id}/votes/users/{user_id} | フィードバックへの投票の取消(Delete Vote For Feedback)
[**get_feedback**](FeedbackApi.md#get_feedback) | **Get** /feedbacks/{feedback_id} | フィードバックの取得(Get Feedback)
[**get_feedback_comment**](FeedbackApi.md#get_feedback_comment) | **Get** /feedbacks/{feedback_id}/comments/{comment_id} | フィードバックへのコメント取得(Get Feedback Comment)
[**get_feedbacks**](FeedbackApi.md#get_feedbacks) | **Get** /feedbacks | フィードバックの一覧を取得(Get Feedbacks)
[**update_feedback**](FeedbackApi.md#update_feedback) | **Patch** /feedbacks/{feedback_id} | フィードバックの編集(Update Feedback)
[**update_feedback_comment**](FeedbackApi.md#update_feedback_comment) | **Patch** /feedbacks/{feedback_id}/comments/{comment_id} | フィードバックへのコメント編集(Update Feedback Comment)
[**update_feedback_status**](FeedbackApi.md#update_feedback_status) | **Patch** /feedbacks/{feedback_id}/status | フィードバックのステータス更新(Update Feedback Status)



## create_feedback

> crate::models::Feedback create_feedback(create_feedback_param)
フィードバックを起票(Create Feedback)

フィードバックを起票します。

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
フィードバックへのコメント投稿(Create Feedback Comment)

フィードバックへのコメントを投稿します。

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
フィードバックへの投票(Create Vote User)

フィードバックへの投票をします。

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
フィードバックを削除(Delete Feedback)

フィードバックを削除します。

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
フィードバックへのコメント削除(Delete Feedback Comment)

フィードバックへのコメントを削除します。

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
フィードバックへの投票の取消(Delete Vote For Feedback)

フィードバックへの投票の取消をします。

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
フィードバックの取得(Get Feedback)

フィードバックの取得をします。

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
フィードバックへのコメント取得(Get Feedback Comment)

フィードバックへのコメントを取得します。

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
フィードバックの一覧を取得(Get Feedbacks)

フィードバックの一覧を取得します。

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
フィードバックの編集(Update Feedback)

フィードバックの編集をします。

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
フィードバックへのコメント編集(Update Feedback Comment)

フィードバックへのコメントを編集します。

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
フィードバックのステータス更新(Update Feedback Status)

フィードバックのステータスを更新します。

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

