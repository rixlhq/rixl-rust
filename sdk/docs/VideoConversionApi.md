# \VideoConversionApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mark_video_failed**](VideoConversionApi.md#mark_video_failed) | **POST** /internal/videos/mark-failed | MarkVideoFailed
[**mark_video_processed**](VideoConversionApi.md#mark_video_processed) | **POST** /internal/videos/mark-processed | MarkVideoProcessed



## mark_video_failed

> serde_json::Value mark_video_failed(videos_v1_mark_video_failed_request)
MarkVideoFailed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**videos_v1_mark_video_failed_request** | [**VideosV1MarkVideoFailedRequest**](VideosV1MarkVideoFailedRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_video_processed

> serde_json::Value mark_video_processed(videos_v1_mark_video_processed_request)
MarkVideoProcessed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**videos_v1_mark_video_processed_request** | [**VideosV1MarkVideoProcessedRequest**](VideosV1MarkVideoProcessedRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

