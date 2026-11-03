# \VideoAnalyticsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_top_videos**](VideoAnalyticsApi.md#get_top_videos) | **GET** /analytics/v1/top/videos | GetTopVideos
[**get_video_stats**](VideoAnalyticsApi.md#get_video_stats) | **GET** /analytics/v1/videos/{video_id}/stats | GetVideoStats



## get_top_videos

> models::AnalyticsV1TopVideosResponse get_top_videos(limit, range_start, range_end)
GetTopVideos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The limit query parameter. |  |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1TopVideosResponse**](analytics.v1.TopVideosResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_stats

> models::AnalyticsV1VideoStats get_video_stats(video_id, range_start, range_end)
GetVideoStats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1VideoStats**](analytics.v1.VideoStats.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

