# \BlogApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blog_subscription**](BlogApi.md#get_blog_subscription) | **GET** /auth/v1/blog/subscription | GetBlogSubscription
[**send_blog_broadcast**](BlogApi.md#send_blog_broadcast) | **POST** /auth/v1/blog/broadcast | SendBlogBroadcast
[**subscribe_blog**](BlogApi.md#subscribe_blog) | **POST** /auth/v1/blog/subscribe | SubscribeBlog
[**unsubscribe_blog**](BlogApi.md#unsubscribe_blog) | **POST** /auth/v1/blog/unsubscribe | UnsubscribeBlog
[**unsubscribe_blog_by_email**](BlogApi.md#unsubscribe_blog_by_email) | **POST** /auth/v1/blog/unsubscribe/email | UnsubscribeBlogByEmail



## get_blog_subscription

> models::AuthV1BlogSubscriptionResponse get_blog_subscription(user_id)
GetBlogSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1BlogSubscriptionResponse**](auth.v1.BlogSubscriptionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_blog_broadcast

> models::AuthV1SendBlogBroadcastResponse send_blog_broadcast(auth_v1_send_blog_broadcast_request)
SendBlogBroadcast

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_send_blog_broadcast_request** | [**AuthV1SendBlogBroadcastRequest**](AuthV1SendBlogBroadcastRequest.md) |  | [required] |

### Return type

[**models::AuthV1SendBlogBroadcastResponse**](auth.v1.SendBlogBroadcastResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_blog

> serde_json::Value subscribe_blog(user_id)
SubscribeBlog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_blog

> serde_json::Value unsubscribe_blog(user_id)
UnsubscribeBlog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_blog_by_email

> serde_json::Value unsubscribe_blog_by_email(auth_v1_unsubscribe_blog_by_email_request)
UnsubscribeBlogByEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_unsubscribe_blog_by_email_request** | [**AuthV1UnsubscribeBlogByEmailRequest**](AuthV1UnsubscribeBlogByEmailRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

