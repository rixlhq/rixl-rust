# \SupportTicketsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_ticket_message**](SupportTicketsApi.md#add_ticket_message) | **POST** /support/v1/tickets/{ticket_id}/messages | AddTicketMessage
[**close_ticket**](SupportTicketsApi.md#close_ticket) | **POST** /support/v1/tickets/{ticket_id}/close | CloseTicket
[**create_ticket**](SupportTicketsApi.md#create_ticket) | **POST** /support/v1/tickets | CreateTicket
[**get_ticket**](SupportTicketsApi.md#get_ticket) | **GET** /support/v1/tickets/{ticket_id} | GetTicket
[**list_categories**](SupportTicketsApi.md#list_categories) | **GET** /support/v1/categories | ListCategories
[**list_tickets**](SupportTicketsApi.md#list_tickets) | **GET** /support/v1/tickets | ListTickets
[**list_topics**](SupportTicketsApi.md#list_topics) | **GET** /support/v1/topics | ListTopics
[**list_topics2**](SupportTicketsApi.md#list_topics2) | **GET** /support/v1/categories/{category_id}/topics | ListTopics



## add_ticket_message

> models::SupportV1AddTicketMessageResponse add_ticket_message(ticket_id, add_ticket_message_request)
AddTicketMessage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** | The ticket_id path parameter. | [required] |
**add_ticket_message_request** | [**AddTicketMessageRequest**](AddTicketMessageRequest.md) |  | [required] |

### Return type

[**models::SupportV1AddTicketMessageResponse**](support.v1.AddTicketMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## close_ticket

> models::SupportV1CloseTicketResponse close_ticket(ticket_id)
CloseTicket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** | The ticket_id path parameter. | [required] |

### Return type

[**models::SupportV1CloseTicketResponse**](support.v1.CloseTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket

> models::SupportV1CreateTicketResponse create_ticket(support_v1_create_ticket_request)
CreateTicket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**support_v1_create_ticket_request** | [**SupportV1CreateTicketRequest**](SupportV1CreateTicketRequest.md) |  | [required] |

### Return type

[**models::SupportV1CreateTicketResponse**](support.v1.CreateTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket

> models::SupportV1GetTicketResponse get_ticket(ticket_id)
GetTicket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** | The ticket_id path parameter. | [required] |

### Return type

[**models::SupportV1GetTicketResponse**](support.v1.GetTicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_categories

> models::SupportV1ListCategoriesResponse list_categories()
ListCategories

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SupportV1ListCategoriesResponse**](support.v1.ListCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tickets

> models::SupportV1ListTicketsResponse list_tickets(pagination_limit, pagination_offset, status)
ListTickets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]
**status** | Option<[**models::SupportV1TicketStatus**](Models__SupportV1TicketStatus.md)> | enum.defined_only = true  |  |

### Return type

[**models::SupportV1ListTicketsResponse**](support.v1.ListTicketsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_topics

> models::SupportV1ListTopicsResponse list_topics(category_id, query)
ListTopics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | Option<**String**> | The category_id query parameter. |  |
**query** | Option<**String**> | The query query parameter. |  |

### Return type

[**models::SupportV1ListTopicsResponse**](support.v1.ListTopicsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_topics2

> models::SupportV1ListTopicsResponse list_topics2(category_id, query)
ListTopics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | The category_id path parameter. | [required] |
**query** | Option<**String**> | The query query parameter. |  |

### Return type

[**models::SupportV1ListTopicsResponse**](support.v1.ListTopicsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

