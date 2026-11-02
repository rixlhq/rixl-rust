# \ImageConversionApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mark_image_failed**](ImageConversionApi.md#mark_image_failed) | **POST** /internal/images/mark-failed | MarkImageFailed
[**mark_image_processed**](ImageConversionApi.md#mark_image_processed) | **POST** /internal/images/mark-processed | MarkImageProcessed
[**take_unprocessed_image**](ImageConversionApi.md#take_unprocessed_image) | **POST** /internal/images/take | TakeUnprocessedImage



## mark_image_failed

> serde_json::Value mark_image_failed(images_v1_mark_image_failed_request)
MarkImageFailed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**images_v1_mark_image_failed_request** | [**ImagesV1MarkImageFailedRequest**](ImagesV1MarkImageFailedRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_image_processed

> serde_json::Value mark_image_processed(images_v1_mark_image_processed_request)
MarkImageProcessed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**images_v1_mark_image_processed_request** | [**ImagesV1MarkImageProcessedRequest**](ImagesV1MarkImageProcessedRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## take_unprocessed_image

> models::ImagesV1UnprocessedImage take_unprocessed_image()
TakeUnprocessedImage

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ImagesV1UnprocessedImage**](images.v1.UnprocessedImage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

