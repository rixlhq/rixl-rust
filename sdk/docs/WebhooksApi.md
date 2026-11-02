# \WebhooksApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_endpoint**](WebhooksApi.md#create_endpoint) | **POST** /webhooks/v1/projects/{project_id}/endpoints | CreateEndpoint
[**delete_endpoint**](WebhooksApi.md#delete_endpoint) | **DELETE** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id} | DeleteEndpoint
[**get_endpoint**](WebhooksApi.md#get_endpoint) | **GET** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id} | GetEndpoint
[**list_endpoint_attempts**](WebhooksApi.md#list_endpoint_attempts) | **GET** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id}/attempts | ListEndpointAttempts
[**list_endpoints**](WebhooksApi.md#list_endpoints) | **GET** /webhooks/v1/projects/{project_id}/endpoints | ListEndpoints
[**list_events**](WebhooksApi.md#list_events) | **GET** /webhooks/v1/projects/{project_id}/events | ListEvents
[**replay_event**](WebhooksApi.md#replay_event) | **POST** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id}/events/{event_id}/replay | ReplayEvent
[**rotate_endpoint_secret**](WebhooksApi.md#rotate_endpoint_secret) | **POST** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id}/rotate-secret | RotateEndpointSecret
[**send_test_event**](WebhooksApi.md#send_test_event) | **POST** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id}/test | SendTestEvent
[**update_endpoint**](WebhooksApi.md#update_endpoint) | **PATCH** /webhooks/v1/projects/{project_id}/endpoints/{endpoint_id} | UpdateEndpoint



## create_endpoint

> models::WebhooksV1CreateEndpointResponse create_endpoint(project_id, create_endpoint_request)
CreateEndpoint

CreateEndpoint registers a new HTTPS endpoint and returns its signing  secret exactly once. Store it; only a rotation returns a secret again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**create_endpoint_request** | [**CreateEndpointRequest**](CreateEndpointRequest.md) |  | [required] |

### Return type

[**models::WebhooksV1CreateEndpointResponse**](webhooks.v1.CreateEndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_endpoint

> serde_json::Value delete_endpoint(project_id, endpoint_id)
DeleteEndpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoint

> models::WebhooksV1GetEndpointResponse get_endpoint(project_id, endpoint_id)
GetEndpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |

### Return type

[**models::WebhooksV1GetEndpointResponse**](webhooks.v1.GetEndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_endpoint_attempts

> models::WebhooksV1ListEndpointAttemptsResponse list_endpoint_attempts(project_id, endpoint_id, pagination_limit, pagination_offset)
ListEndpointAttempts

ListEndpointAttempts returns the delivery history for one endpoint,  newest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::WebhooksV1ListEndpointAttemptsResponse**](webhooks.v1.ListEndpointAttemptsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_endpoints

> models::WebhooksV1ListEndpointsResponse list_endpoints(project_id, pagination_limit, pagination_offset)
ListEndpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::WebhooksV1ListEndpointsResponse**](webhooks.v1.ListEndpointsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_events

> models::WebhooksV1ListEventsResponse list_events(project_id, pagination_limit, pagination_offset)
ListEvents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::WebhooksV1ListEventsResponse**](webhooks.v1.ListEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replay_event

> serde_json::Value replay_event(project_id, endpoint_id, event_id)
ReplayEvent

ReplayEvent re-delivers one event to one endpoint with the same  webhook-id so consumers dedupe it against the original delivery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |
**event_id** | **String** | The event_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_endpoint_secret

> models::WebhooksV1RotateEndpointSecretResponse rotate_endpoint_secret(project_id, endpoint_id)
RotateEndpointSecret

RotateEndpointSecret installs a fresh signing secret. For 24h deliveries  are signed with both the old and the new secret so consumers can migrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |

### Return type

[**models::WebhooksV1RotateEndpointSecretResponse**](webhooks.v1.RotateEndpointSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_event

> models::WebhooksV1SendTestEventResponse send_test_event(project_id, endpoint_id)
SendTestEvent

SendTestEvent delivers one synthetic webhook.test event to the endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |

### Return type

[**models::WebhooksV1SendTestEventResponse**](webhooks.v1.SendTestEventResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_endpoint

> models::WebhooksV1UpdateEndpointResponse update_endpoint(project_id, endpoint_id, update_endpoint_request)
UpdateEndpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**endpoint_id** | **String** | The endpoint_id path parameter. | [required] |
**update_endpoint_request** | [**UpdateEndpointRequest**](UpdateEndpointRequest.md) |  | [required] |

### Return type

[**models::WebhooksV1UpdateEndpointResponse**](webhooks.v1.UpdateEndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

