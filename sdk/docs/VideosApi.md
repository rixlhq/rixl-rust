# \VideosApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete**](VideosApi.md#delete) | **DELETE** /videos/{videoId}/delete | Delete video
[**delete_audio_track**](VideosApi.md#delete_audio_track) | **DELETE** /videos/{videoId}/audio-tracks/{trackId} | Delete audio track
[**delete_audio_track_by_language**](VideosApi.md#delete_audio_track_by_language) | **DELETE** /videos/{videoId}/audio-tracks/language/{lang_code} | Delete audio track by language
[**delete_audio_tracks**](VideosApi.md#delete_audio_tracks) | **DELETE** /videos/{videoId}/audio-tracks | Delete all audio tracks
[**delete_chapters**](VideosApi.md#delete_chapters) | **DELETE** /videos/{videoId}/chapters | Delete video chapters
[**delete_subtitle**](VideosApi.md#delete_subtitle) | **DELETE** /videos/{videoId}/subtitles/{subtitleId} | Delete subtitle
[**delete_subtitle_by_language**](VideosApi.md#delete_subtitle_by_language) | **DELETE** /videos/{videoId}/subtitles/language/{lang_code} | Delete subtitle by language
[**delete_subtitles**](VideosApi.md#delete_subtitles) | **DELETE** /videos/{videoId}/subtitles | Delete all subtitles
[**get**](VideosApi.md#get) | **GET** /videos/{videoId} | Get a video
[**list**](VideosApi.md#list) | **GET** /videos | List videos for a project
[**list_languages**](VideosApi.md#list_languages) | **GET** /videos/languages | List available subtitle languages
[**replace_audio_tracks**](VideosApi.md#replace_audio_tracks) | **POST** /videos/{videoId}/audio-tracks | Bulk upsert video audio tracks
[**replace_subtitles**](VideosApi.md#replace_subtitles) | **POST** /videos/{videoId}/subtitles | Bulk upsert video subtitles
[**update_audio_track_by_language**](VideosApi.md#update_audio_track_by_language) | **PUT** /videos/{videoId}/audio-tracks/language/{lang_code} | Upsert video audio track
[**update_chapters**](VideosApi.md#update_chapters) | **PUT** /videos/{videoId}/chapters | Update video chapters
[**update_subtitle_by_language**](VideosApi.md#update_subtitle_by_language) | **PUT** /videos/{videoId}/subtitles/language/{lang_code} | Upsert video subtitle
[**update_thumbnail**](VideosApi.md#update_thumbnail) | **PUT** /videos/{videoId}/thumbnail | Update video thumbnail
[**upload_complete**](VideosApi.md#upload_complete) | **POST** /videos/upload/complete | Upload: Mark as complete
[**upload_init**](VideosApi.md#upload_init) | **POST** /videos/upload/init | Upload: Init



## delete

> delete(video_id)
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


## delete_audio_track

> models::AudioTrackDelete delete_audio_track(video_id, track_id)
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


## delete_subtitle

> models::SubtitleDelete delete_subtitle(video_id, subtitle_id)
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


## get

> models::Video get(video_id)
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


## list

> models::PaginationPaginatedResponseVideo list(limit, offset, sort, order)
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


## list_languages

> Vec<models::SubtitleLanguageResponse> list_languages()
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


## update_thumbnail

> models::Video update_thumbnail(video_id, thumbnail)
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


## upload_complete

> models::Video upload_complete(video_upload_complete_request)
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


## upload_init

> models::VideoUploadInitResponse upload_init(video_upload_init_request)
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

