# \SubtitlesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subtitle_upload**](SubtitlesApi.md#create_subtitle_upload) | **POST** /media/v1/projects/{project_id}/videos/{video_id}/subtitles/upload | CreateSubtitleUpload
[**delete_all_subtitles**](SubtitlesApi.md#delete_all_subtitles) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/subtitles | DeleteAllSubtitles
[**delete_subtitle**](SubtitlesApi.md#delete_subtitle) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/subtitles/{subtitle_id} | DeleteSubtitle
[**delete_subtitles_by_language**](SubtitlesApi.md#delete_subtitles_by_language) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/subtitles/language/{language_code} | DeleteSubtitlesByLanguage
[**list_subtitles**](SubtitlesApi.md#list_subtitles) | **GET** /media/v1/videos/{video_id}/subtitles | ListSubtitles



## create_subtitle_upload

> models::VideosV1TrackUpload create_subtitle_upload(project_id, video_id, create_track_upload_request1)
CreateSubtitleUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**create_track_upload_request1** | [**CreateTrackUploadRequest1**](CreateTrackUploadRequest1.md) |  | [required] |

### Return type

[**models::VideosV1TrackUpload**](videos.v1.TrackUpload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_subtitles

> serde_json::Value delete_all_subtitles(project_id, video_id)
DeleteAllSubtitles

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


## delete_subtitle

> serde_json::Value delete_subtitle(project_id, video_id, subtitle_id)
DeleteSubtitle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**subtitle_id** | **String** | The subtitle_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subtitles_by_language

> serde_json::Value delete_subtitles_by_language(project_id, video_id, language_code)
DeleteSubtitlesByLanguage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**language_code** | **String** | The language_code path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subtitles

> models::VideosV1ListSubtitlesResponse list_subtitles(video_id)
ListSubtitles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |

### Return type

[**models::VideosV1ListSubtitlesResponse**](videos.v1.ListSubtitlesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

