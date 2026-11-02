# \PostsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_post**](PostsApi.md#create_post) | **POST** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts | CreatePost
[**create_post_upload**](PostsApi.md#create_post_upload) | **POST** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts/upload | CreatePostUpload
[**delete_post**](PostsApi.md#delete_post) | **DELETE** /posts/v1/projects/{project_id}/posts/{post_id} | DeletePost
[**delete_post2**](PostsApi.md#delete_post2) | **DELETE** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts/{post_id} | DeletePost
[**get_post**](PostsApi.md#get_post) | **GET** /posts/v1/projects/{project_id}/posts/{post_id} | GetPost
[**get_post2**](PostsApi.md#get_post2) | **GET** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts/{post_id} | GetPost
[**get_post3**](PostsApi.md#get_post3) | **GET** /posts/v1/feeds/{feed_id}/{post_id} | GetPost
[**list_posts**](PostsApi.md#list_posts) | **GET** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts | ListPosts
[**list_posts2**](PostsApi.md#list_posts2) | **GET** /posts/v1/projects/{project_id}/feeds/{feed_id}/posts/creators/{creator_id} | ListPosts
[**list_posts3**](PostsApi.md#list_posts3) | **GET** /posts/v1/feeds/{feed_id} | ListPosts
[**list_posts4**](PostsApi.md#list_posts4) | **GET** /posts/v1/feeds/{feed_id}/creators/{creator_id} | ListPosts



## create_post

> models::PostsV1CreatePostResponse create_post(project_id, feed_id, create_post_request)
CreatePost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |
**create_post_request** | [**CreatePostRequest**](CreatePostRequest.md) |  | [required] |

### Return type

[**models::PostsV1CreatePostResponse**](posts.v1.CreatePostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_post_upload

> models::PostsV1PostUpload create_post_upload(project_id, feed_id, create_post_upload_request)
CreatePostUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |
**create_post_upload_request** | [**CreatePostUploadRequest**](CreatePostUploadRequest.md) |  | [required] |

### Return type

[**models::PostsV1PostUpload**](posts.v1.PostUpload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_post

> serde_json::Value delete_post(project_id, post_id, feed_id)
DeletePost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**post_id** | **String** | The post_id path parameter. | [required] |
**feed_id** | Option<**String**> | feed_id is accepted from the feed-scoped delete path but not used by the  backend (deletion is keyed by post_id). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_post2

> serde_json::Value delete_post2(project_id, feed_id, post_id)
DeletePost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | feed_id is accepted from the feed-scoped delete path but not used by the  backend (deletion is keyed by post_id). | [required] |
**post_id** | **String** | The post_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_post

> models::PostsV1GetPostResponse get_post(project_id, post_id, feed_id)
GetPost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**post_id** | **String** | The post_id path parameter. | [required] |
**feed_id** | Option<**String**> | Populated from feed-scoped routes; the handler resolves by post_id, so this  is used only for path binding / optional validation. |  |

### Return type

[**models::PostsV1GetPostResponse**](posts.v1.GetPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_post2

> models::PostsV1GetPostResponse get_post2(project_id, feed_id, post_id)
GetPost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | Populated from feed-scoped routes; the handler resolves by post_id, so this  is used only for path binding / optional validation. | [required] |
**post_id** | **String** | The post_id path parameter. | [required] |

### Return type

[**models::PostsV1GetPostResponse**](posts.v1.GetPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_post3

> models::PostsV1GetPostResponse get_post3(feed_id, post_id, project_id)
GetPost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | Populated from feed-scoped routes; the handler resolves by post_id, so this  is used only for path binding / optional validation. | [required] |
**post_id** | **String** | The post_id path parameter. | [required] |
**project_id** | Option<**String**> | The project_id query parameter. |  |

### Return type

[**models::PostsV1GetPostResponse**](posts.v1.GetPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_posts

> models::PostsV1ListPostsResponse list_posts(project_id, feed_id, pagination_limit, pagination_offset, creator_id)
ListPosts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]
**creator_id** | Option<**String**> | The creator_id query parameter. |  |

### Return type

[**models::PostsV1ListPostsResponse**](posts.v1.ListPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_posts2

> models::PostsV1ListPostsResponse list_posts2(project_id, feed_id, creator_id, pagination_limit, pagination_offset)
ListPosts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |
**creator_id** | **String** | The creator_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::PostsV1ListPostsResponse**](posts.v1.ListPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_posts3

> models::PostsV1ListPostsResponse list_posts3(feed_id, project_id, pagination_limit, pagination_offset, creator_id)
ListPosts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | The feed_id path parameter. | [required] |
**project_id** | Option<**String**> | The project_id query parameter. |  |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]
**creator_id** | Option<**String**> | The creator_id query parameter. |  |

### Return type

[**models::PostsV1ListPostsResponse**](posts.v1.ListPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_posts4

> models::PostsV1ListPostsResponse list_posts4(feed_id, creator_id, project_id, pagination_limit, pagination_offset)
ListPosts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | The feed_id path parameter. | [required] |
**creator_id** | **String** | The creator_id path parameter. | [required] |
**project_id** | Option<**String**> | The project_id query parameter. |  |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::PostsV1ListPostsResponse**](posts.v1.ListPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

