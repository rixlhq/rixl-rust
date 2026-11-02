# \EventsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_events**](EventsApi.md#track_events) | **POST** /analytics/v1/events | TrackEvents



## track_events

> serde_json::Value track_events(analytics_v1_track_events_request)
TrackEvents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_track_events_request** | [**AnalyticsV1TrackEventsRequest**](AnalyticsV1TrackEventsRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

