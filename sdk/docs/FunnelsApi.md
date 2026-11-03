# \FunnelsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_funnel_analytics**](FunnelsApi.md#get_funnel_analytics) | **POST** /analytics/v1/funnels | GetFunnelAnalytics
[**get_retention_analytics**](FunnelsApi.md#get_retention_analytics) | **POST** /analytics/v1/retention | GetRetentionAnalytics



## get_funnel_analytics

> models::AnalyticsV1FunnelAnalytics get_funnel_analytics(analytics_v1_funnel_request)
GetFunnelAnalytics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_funnel_request** | [**AnalyticsV1FunnelRequest**](AnalyticsV1FunnelRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1FunnelAnalytics**](analytics.v1.FunnelAnalytics.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention_analytics

> models::AnalyticsV1RetentionAnalytics get_retention_analytics(analytics_v1_get_retention_request)
GetRetentionAnalytics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_get_retention_request** | [**AnalyticsV1GetRetentionRequest**](AnalyticsV1GetRetentionRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1RetentionAnalytics**](analytics.v1.RetentionAnalytics.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

