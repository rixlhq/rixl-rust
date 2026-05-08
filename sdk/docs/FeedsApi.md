# \FeedsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_feed_post**](FeedsApi.md#get_feed_post) | **GET** /feeds/{feedId}/{postId} | Get a post
[**list_feed_posts**](FeedsApi.md#list_feed_posts) | **GET** /feeds/{feedId} | List posts in a feed
[**list_feed_posts_by_creator**](FeedsApi.md#list_feed_posts_by_creator) | **GET** /feeds/{feedId}/creators/{creatorId} | List posts by creator



## get_feed_post

> models::Post get_feed_post(feed_id, post_id)
Get a post

Retrieve a post from feed by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | Feed ID | [required] |
**post_id** | **String** | Post ID | [required] |

### Return type

[**models::Post**](Post.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_feed_posts

> models::PaginationPaginatedResponsePost list_feed_posts(feed_id, limit, offset)
List posts in a feed

Retrieve posts in a feed, with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | Feed ID | [required] |
**limit** | Option<**i32**> | Maximum number of items to return in a single request. <br> **Default:** `25` |  |[default to 25]
**offset** | Option<**i32**> | Starting point of the result set. <br>To get page 2 with a limit of 25, set `offset` to `25`. <br> **Default:** `0` |  |[default to 0]

### Return type

[**models::PaginationPaginatedResponsePost**](pagination.PaginatedResponse-Post.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_feed_posts_by_creator

> models::PaginationPaginatedResponsePost list_feed_posts_by_creator(feed_id, creator_id, limit, offset)
List posts by creator

Retrieve posts in a feed by a specific creator, with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | Feed ID | [required] |
**creator_id** | **String** | Creator ID | [required] |
**limit** | Option<**i32**> | Maximum number of items to return in a single request. <br> **Default:** `25` |  |[default to 25]
**offset** | Option<**i32**> | Starting point of the result set. <br>To get page 2 with a limit of 25, set `offset` to `25`. <br> **Default:** `0` |  |[default to 0]

### Return type

[**models::PaginationPaginatedResponsePost**](pagination.PaginatedResponse-Post.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

