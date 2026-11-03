# \PostAnalyticsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_post_stats**](PostAnalyticsApi.md#get_post_stats) | **GET** /analytics/v1/posts/{post_id}/stats | GetPostStats
[**get_top_posts**](PostAnalyticsApi.md#get_top_posts) | **GET** /analytics/v1/top/posts | GetTopPosts



## get_post_stats

> models::AnalyticsV1GetPostStatsResponse get_post_stats(post_id, range_start, range_end)
GetPostStats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | The post_id path parameter. | [required] |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1GetPostStatsResponse**](analytics.v1.GetPostStatsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_posts

> models::AnalyticsV1TopPostsResponse get_top_posts(limit, range_start, range_end, feed_id)
GetTopPosts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The limit query parameter. |  |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |
**feed_id** | Option<**String**> | The feed_id query parameter. |  |

### Return type

[**models::AnalyticsV1TopPostsResponse**](analytics.v1.TopPostsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

