# \ChaptersApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_video_chapter**](ChaptersApi.md#delete_video_chapter) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/chapters/{start_time_sec} | DeleteVideoChapter
[**get_video_chapters**](ChaptersApi.md#get_video_chapters) | **GET** /media/v1/projects/{project_id}/videos/{video_id}/chapters | GetVideoChapters
[**update_video_chapters**](ChaptersApi.md#update_video_chapters) | **PUT** /media/v1/projects/{project_id}/videos/{video_id}/chapters | UpdateVideoChapters
[**update_video_chapters2**](ChaptersApi.md#update_video_chapters2) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/chapters | UpdateVideoChapters



## delete_video_chapter

> models::VideosV1VideoChapters delete_video_chapter(project_id, video_id, start_time_sec)
DeleteVideoChapter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**start_time_sec** | **String** | The start_time_sec path parameter. | [required] |

### Return type

[**models::VideosV1VideoChapters**](videos.v1.VideoChapters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video_chapters

> models::VideosV1VideoChapters get_video_chapters(project_id, video_id)
GetVideoChapters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |

### Return type

[**models::VideosV1VideoChapters**](videos.v1.VideoChapters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_chapters

> models::VideosV1VideoChapters update_video_chapters(project_id, video_id, update_video_chapters_request)
UpdateVideoChapters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**update_video_chapters_request** | [**UpdateVideoChaptersRequest**](UpdateVideoChaptersRequest.md) |  | [required] |

### Return type

[**models::VideosV1VideoChapters**](videos.v1.VideoChapters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_chapters2

> models::VideosV1VideoChapters update_video_chapters2(project_id, video_id, chapters_title, chapters_start_time_sec)
UpdateVideoChapters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**chapters_title** | Option<**String**> | The chapters.title query parameter. |  |
**chapters_start_time_sec** | Option<**f64**> | The chapters.start_time_sec query parameter. |  |

### Return type

[**models::VideosV1VideoChapters**](videos.v1.VideoChapters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

