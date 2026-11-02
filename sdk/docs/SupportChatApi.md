# \SupportChatApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_chat_messages**](SupportChatApi.md#list_chat_messages) | **GET** /support/v1/chat/messages | ListChatMessages
[**send_chat_message**](SupportChatApi.md#send_chat_message) | **POST** /support/v1/chat/messages | SendChatMessage



## list_chat_messages

> models::SupportV1ListChatMessagesResponse list_chat_messages(after, limit)
ListChatMessages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. |  |
**limit** | Option<**i32**> | The limit query parameter. |  |

### Return type

[**models::SupportV1ListChatMessagesResponse**](support.v1.ListChatMessagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_message

> models::SupportV1SendChatMessageResponse send_chat_message(support_v1_send_chat_message_request)
SendChatMessage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**support_v1_send_chat_message_request** | [**SupportV1SendChatMessageRequest**](SupportV1SendChatMessageRequest.md) |  | [required] |

### Return type

[**models::SupportV1SendChatMessageResponse**](support.v1.SendChatMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

