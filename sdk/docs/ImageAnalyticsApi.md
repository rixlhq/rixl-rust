# \ImageAnalyticsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_image_stats**](ImageAnalyticsApi.md#get_image_stats) | **GET** /analytics/v1/images/{image_id}/stats | GetImageStats
[**get_top_images**](ImageAnalyticsApi.md#get_top_images) | **GET** /analytics/v1/top/images | GetTopImages



## get_image_stats

> models::AnalyticsV1ImageStats get_image_stats(image_id, range_start, range_end)
GetImageStats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | The image_id path parameter. | [required] |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1ImageStats**](analytics.v1.ImageStats.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_images

> models::AnalyticsV1TopImagesResponse get_top_images(limit, range_start, range_end)
GetTopImages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The limit query parameter. |  |
**range_start** | Option<**String**> | The range.start query parameter. |  |
**range_end** | Option<**String**> | The range.end query parameter. |  |

### Return type

[**models::AnalyticsV1TopImagesResponse**](analytics.v1.TopImagesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

