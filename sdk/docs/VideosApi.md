# \VideosApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_video_upload**](VideosApi.md#create_video_upload) | **POST** /media/v1/projects/{project_id}/videos/upload | CreateVideoUpload
[**delete_video**](VideosApi.md#delete_video) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id} | DeleteVideo
[**get_video**](VideosApi.md#get_video) | **GET** /media/v1/videos/{video_id} | GetVideo
[**list_videos**](VideosApi.md#list_videos) | **GET** /media/v1/projects/{project_id}/videos | ListVideos
[**update_video_visibility**](VideosApi.md#update_video_visibility) | **PATCH** /media/v1/projects/{project_id}/videos/{video_id}/visibility | UpdateVideoVisibility



## create_video_upload

> models::VideosV1VideoUpload create_video_upload(project_id, create_video_upload_request)
CreateVideoUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**create_video_upload_request** | [**CreateVideoUploadRequest**](CreateVideoUploadRequest.md) |  | [required] |

### Return type

[**models::VideosV1VideoUpload**](videos.v1.VideoUpload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_video

> serde_json::Value delete_video(project_id, video_id)
DeleteVideo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video

> models::VideosV1GetVideoResponse get_video(video_id)
GetVideo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |

### Return type

[**models::VideosV1GetVideoResponse**](videos.v1.GetVideoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_videos

> models::VideosV1ListVideosResponse list_videos(project_id, pagination_limit, pagination_offset, sort_field, sort_direction)
ListVideos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]
**sort_field** | Option<**String**> | The sort_field query parameter. |  |
**sort_direction** | Option<**String**> | The sort_direction query parameter. |  |

### Return type

[**models::VideosV1ListVideosResponse**](videos.v1.ListVideosResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_visibility

> models::VideosV1UpdateVideoVisibilityResponse update_video_visibility(project_id, video_id, update_video_visibility_request)
UpdateVideoVisibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**update_video_visibility_request** | [**UpdateVideoVisibilityRequest**](UpdateVideoVisibilityRequest.md) |  | [required] |

### Return type

[**models::VideosV1UpdateVideoVisibilityResponse**](videos.v1.UpdateVideoVisibilityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

