# \VideosApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_video_upload**](VideosApi.md#complete_video_upload) | **POST** /videos/upload/complete | Upload: Mark as complete
[**delete_audio_track_by_id**](VideosApi.md#delete_audio_track_by_id) | **DELETE** /videos/{videoId}/audio-tracks/{trackId} | Delete audio track
[**delete_audio_track_by_language**](VideosApi.md#delete_audio_track_by_language) | **DELETE** /videos/{videoId}/audio-tracks/language/{lang_code} | Delete audio track by language
[**delete_audio_tracks**](VideosApi.md#delete_audio_tracks) | **DELETE** /videos/{videoId}/audio-tracks | Delete all audio tracks
[**delete_chapters**](VideosApi.md#delete_chapters) | **DELETE** /videos/{videoId}/chapters | Delete video chapters
[**delete_subtitle_by_id**](VideosApi.md#delete_subtitle_by_id) | **DELETE** /videos/{videoId}/subtitles/{subtitleId} | Delete subtitle
[**delete_subtitle_by_language**](VideosApi.md#delete_subtitle_by_language) | **DELETE** /videos/{videoId}/subtitles/language/{lang_code} | Delete subtitle by language
[**delete_subtitles**](VideosApi.md#delete_subtitles) | **DELETE** /videos/{videoId}/subtitles | Delete all subtitles
[**delete_video**](VideosApi.md#delete_video) | **DELETE** /videos/{videoId}/delete | Delete video
[**get_video**](VideosApi.md#get_video) | **GET** /videos/{videoId} | Get a video
[**init_video_upload**](VideosApi.md#init_video_upload) | **POST** /videos/upload/init | Upload: Init
[**list_video_languages**](VideosApi.md#list_video_languages) | **GET** /videos/languages | List available subtitle languages
[**list_videos**](VideosApi.md#list_videos) | **GET** /videos | List videos for a project
[**replace_audio_tracks**](VideosApi.md#replace_audio_tracks) | **POST** /videos/{videoId}/audio-tracks | Bulk upsert video audio tracks
[**replace_subtitles**](VideosApi.md#replace_subtitles) | **POST** /videos/{videoId}/subtitles | Bulk upsert video subtitles
[**update_audio_track_by_language**](VideosApi.md#update_audio_track_by_language) | **PUT** /videos/{videoId}/audio-tracks/language/{lang_code} | Upsert video audio track
[**update_chapters**](VideosApi.md#update_chapters) | **PUT** /videos/{videoId}/chapters | Update video chapters
[**update_subtitle_by_language**](VideosApi.md#update_subtitle_by_language) | **PUT** /videos/{videoId}/subtitles/language/{lang_code} | Upsert video subtitle
[**update_video_thumbnail**](VideosApi.md#update_video_thumbnail) | **PUT** /videos/{videoId}/thumbnail | Update video thumbnail



## complete_video_upload

> models::Video complete_video_upload(video_upload_complete_request)
Upload: Mark as complete

Mark a video upload as complete after successful upload to storage using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_upload_complete_request** | [**VideoUploadCompleteRequest**](VideoUploadCompleteRequest.md) | Video upload completion request | [required] |

### Return type

[**models::Video**](Video.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_audio_track_by_id

> models::AudioTrackDelete delete_audio_track_by_id(video_id, track_id)
Delete audio track

Remove an additional audio track from a video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**track_id** | **String** | Audio Track ID | [required] |

### Return type

[**models::AudioTrackDelete**](AudioTrackDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_audio_track_by_language

> models::AudioTrackDelete delete_audio_track_by_language(video_id, lang_code)
Delete audio track by language

Remove an audio track for a specific language using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**lang_code** | **String** | Language Code (BCP 47) | [required] |

### Return type

[**models::AudioTrackDelete**](AudioTrackDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_audio_tracks

> models::AudioTrackDelete delete_audio_tracks(video_id)
Delete all audio tracks

Remove all additional audio tracks from a video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |

### Return type

[**models::AudioTrackDelete**](AudioTrackDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chapters

> models::UpdateChaptersResponse delete_chapters(video_id)
Delete video chapters

Remove all chapters from a video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |

### Return type

[**models::UpdateChaptersResponse**](UpdateChaptersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subtitle_by_id

> models::SubtitleDelete delete_subtitle_by_id(video_id, subtitle_id)
Delete subtitle

Remove a subtitle from a video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**subtitle_id** | **String** | Subtitle ID | [required] |

### Return type

[**models::SubtitleDelete**](SubtitleDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subtitle_by_language

> models::SubtitleDelete delete_subtitle_by_language(video_id, lang_code)
Delete subtitle by language

Remove a subtitle for a specific language using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**lang_code** | **String** | Language Code (BCP 47) | [required] |

### Return type

[**models::SubtitleDelete**](SubtitleDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subtitles

> models::SubtitleDelete delete_subtitles(video_id)
Delete all subtitles

Remove all subtitles from a video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |

### Return type

[**models::SubtitleDelete**](SubtitleDelete.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_video

> delete_video(video_id)
Delete video

Delete a video by its ID within a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_video

> models::Video get_video(video_id)
Get a video

Retrieve a video by its ID for a specific project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |

### Return type

[**models::Video**](Video.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## init_video_upload

> models::VideoUploadInitResponse init_video_upload(video_upload_init_request)
Upload: Init

Initialize a video upload and get presigned URLs for video and poster using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_upload_init_request** | [**VideoUploadInitRequest**](VideoUploadInitRequest.md) | Video upload initialization request | [required] |

### Return type

[**models::VideoUploadInitResponse**](github_com_rixlhq_api_internal_videos_handler_upload.InitResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_video_languages

> Vec<models::SubtitleLanguageResponse> list_video_languages()
List available subtitle languages

Get list of supported languages for subtitles

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SubtitleLanguageResponse>**](internal_videos_handler_subtitles.languageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_videos

> models::PaginationPaginatedResponseVideo list_videos(limit, offset, sort, order)
List videos for a project

Retrieve all videos for a specific project, with pagination and sorting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of items to return in a single request. <br> **Default:** `25` |  |[default to 25]
**offset** | Option<**i32**> | Starting point of the result set. <br>To get page 2 with a limit of 25, set `offset` to `25`. <br> **Default:** `0` |  |[default to 0]
**sort** | Option<**String**> | Field to sort by (created_at, name, size, updated_at, duration) |  |
**order** | Option<**String**> | Sort order (asc, desc) |  |

### Return type

[**models::PaginationPaginatedResponseVideo**](pagination.PaginatedResponse-Video.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_audio_tracks

> Vec<models::AudioTrack> replace_audio_tracks(video_id, files, labels, language_codes)
Bulk upsert video audio tracks

Replace all audio tracks with the provided ones using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**files** | [**Vec<std::path::PathBuf>**](Std__path__PathBuf.md) | Audio files (.mp3, .opus, .flac, .wav, .ac3, .m4a, .aac) | [required] |
**labels** | **String** | Comma-separated labels | [required] |
**language_codes** | **String** | Comma-separated language codes | [required] |

### Return type

[**Vec<models::AudioTrack>**](AudioTrack.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_subtitles

> Vec<models::Subtitle> replace_subtitles(video_id, files, labels, language_codes)
Bulk upsert video subtitles

Replace all subtitles with the provided ones using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**files** | [**Vec<std::path::PathBuf>**](Std__path__PathBuf.md) | Subtitle files (.srt or .vtt) | [required] |
**labels** | **String** | Comma-separated labels | [required] |
**language_codes** | **String** | Comma-separated language codes | [required] |

### Return type

[**Vec<models::Subtitle>**](Subtitle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_audio_track_by_language

> models::AudioTrack update_audio_track_by_language(video_id, lang_code, file, label)
Upsert video audio track

Add or replace an audio track for a specific language using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**lang_code** | **String** | Language Code (BCP 47) | [required] |
**file** | **std::path::PathBuf** | Audio file (.mp3, .opus, .flac, .wav, .ac3, .m4a, .aac) | [required] |
**label** | Option<**String**> | Label (e.g. English) |  |

### Return type

[**models::AudioTrack**](AudioTrack.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chapters

> models::UpdateChaptersResponse update_chapters(video_id, update_chapters_request)
Update video chapters

Replace all chapters for a video (atomic PUT operation) using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**update_chapters_request** | [**UpdateChaptersRequest**](UpdateChaptersRequest.md) | Chapters array | [required] |

### Return type

[**models::UpdateChaptersResponse**](UpdateChaptersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subtitle_by_language

> models::Subtitle update_subtitle_by_language(video_id, lang_code, file, label)
Upsert video subtitle

Add or replace a subtitle for a specific language using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**lang_code** | **String** | Language Code (BCP 47) | [required] |
**file** | **std::path::PathBuf** | Subtitle file (.srt or .vtt) | [required] |
**label** | Option<**String**> | Label (e.g. English) |  |

### Return type

[**models::Subtitle**](Subtitle.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_thumbnail

> models::Video update_video_thumbnail(video_id, thumbnail)
Update video thumbnail

Update the thumbnail image for an existing video using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**video_id** | **String** | Video ID | [required] |
**thumbnail** | **std::path::PathBuf** | Thumbnail image file (max 5MB, image/_*) | [required] |

### Return type

[**models::Video**](Video.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

