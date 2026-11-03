# \FeedsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feed**](FeedsApi.md#create_feed) | **POST** /feeds/v1/projects/{project_id}/feeds | CreateFeed
[**delete_feed**](FeedsApi.md#delete_feed) | **DELETE** /feeds/v1/projects/{project_id}/feeds/{feed_id} | DeleteFeed
[**get_feed**](FeedsApi.md#get_feed) | **GET** /feeds/v1/projects/{project_id}/feeds/{feed_id} | GetFeed
[**list_feeds**](FeedsApi.md#list_feeds) | **GET** /feeds/v1/projects/{project_id}/feeds | ListFeeds
[**update_feed**](FeedsApi.md#update_feed) | **PUT** /feeds/v1/projects/{project_id}/feeds/{feed_id} | UpdateFeed



## create_feed

> models::FeedsV1Feed create_feed(project_id, create_feed_request)
CreateFeed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**create_feed_request** | [**CreateFeedRequest**](CreateFeedRequest.md) |  | [required] |

### Return type

[**models::FeedsV1Feed**](feeds.v1.Feed.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feed

> serde_json::Value delete_feed(project_id, feed_id)
DeleteFeed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed

> models::FeedsV1Feed get_feed(project_id, feed_id)
GetFeed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |

### Return type

[**models::FeedsV1Feed**](feeds.v1.Feed.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_feeds

> models::FeedsV1ListFeedsResponse list_feeds(project_id, pagination_limit, pagination_offset)
ListFeeds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::FeedsV1ListFeedsResponse**](feeds.v1.ListFeedsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feed

> models::FeedsV1Feed update_feed(project_id, feed_id, update_feed_request)
UpdateFeed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**feed_id** | **String** | The feed_id path parameter. | [required] |
**update_feed_request** | [**UpdateFeedRequest**](UpdateFeedRequest.md) |  | [required] |

### Return type

[**models::FeedsV1Feed**](feeds.v1.Feed.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

