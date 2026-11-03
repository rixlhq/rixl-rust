# \AudioTracksApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_audio_track_upload**](AudioTracksApi.md#create_audio_track_upload) | **POST** /media/v1/projects/{project_id}/videos/{video_id}/audio-tracks/upload | CreateAudioTrackUpload
[**delete_all_audio_tracks**](AudioTracksApi.md#delete_all_audio_tracks) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/audio-tracks | DeleteAllAudioTracks
[**delete_audio_track**](AudioTracksApi.md#delete_audio_track) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/audio-tracks/{track_id} | DeleteAudioTrack
[**delete_audio_tracks_by_language**](AudioTracksApi.md#delete_audio_tracks_by_language) | **DELETE** /media/v1/projects/{project_id}/videos/{video_id}/audio-tracks/language/{language_code} | DeleteAudioTracksByLanguage
[**list_audio_tracks**](AudioTracksApi.md#list_audio_tracks) | **GET** /media/v1/videos/{video_id}/audio-tracks | ListAudioTracks



## create_audio_track_upload

> models::VideosV1TrackUpload create_audio_track_upload(project_id, video_id, create_track_upload_request)
CreateAudioTrackUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**create_track_upload_request** | [**CreateTrackUploadRequest**](CreateTrackUploadRequest.md) |  | [required] |

### Return type

[**models::VideosV1TrackUpload**](videos.v1.TrackUpload.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_audio_tracks

> serde_json::Value delete_all_audio_tracks(project_id, video_id)
DeleteAllAudioTracks

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


## delete_audio_track

> serde_json::Value delete_audio_track(project_id, video_id, track_id)
DeleteAudioTrack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**video_id** | **String** | The video_id path parameter. | [required] |
**track_id** | **String** | The track_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_audio_tracks_by_language

> serde_json::Value delete_audio_tracks_by_language(project_id, video_id, language_code)
DeleteAudioTracksByLanguage

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


## list_audio_tracks

> models::VideosV1ListAudioTracksResponse list_audio_tracks(video_id)
ListAudioTracks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | The video_id path parameter. | [required] |

### Return type

[**models::VideosV1ListAudioTracksResponse**](videos.v1.ListAudioTracksResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

