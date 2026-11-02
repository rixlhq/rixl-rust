# \FeedAnalyticsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_feed_stats**](FeedAnalyticsApi.md#get_feed_stats) | **GET** /analytics/v1/feeds/{feed_id}/stats | GetFeedStats
[**get_top_feeds**](FeedAnalyticsApi.md#get_top_feeds) | **GET** /analytics/v1/top/feeds | GetTopFeeds



## get_feed_stats

> models::AnalyticsV1FeedStats get_feed_stats(feed_id, range_start, range_end)
GetFeedStats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | The feed_id path parameter. | [required] |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1FeedStats**](analytics.v1.FeedStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_feeds

> models::AnalyticsV1TopFeedsResponse get_top_feeds(limit, range_start, range_end)
GetTopFeeds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The limit query parameter. |  |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1TopFeedsResponse**](analytics.v1.TopFeedsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

