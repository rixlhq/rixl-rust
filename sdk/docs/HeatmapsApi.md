# \HeatmapsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_hot_segments**](HeatmapsApi.md#get_hot_segments) | **GET** /analytics/v1/videos/{video_id}/hot-segments | GetHotSegments
[**get_video_heatmap**](HeatmapsApi.md#get_video_heatmap) | **GET** /analytics/v1/videos/{video_id}/heatmap | GetVideoHeatmap



## get_hot_segments

> models::AnalyticsV1HotSegmentsResponse get_hot_segments(video_id, range_start, range_end)
GetHotSegments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1HotSegmentsResponse**](analytics.v1.HotSegmentsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_heatmap

> models::AnalyticsV1VideoHeatmap get_video_heatmap(video_id, buckets, range_start, range_end)
GetVideoHeatmap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |
**buckets** | Option<**i32**> | The buckets query parameter. |  |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1VideoHeatmap**](analytics.v1.VideoHeatmap.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

