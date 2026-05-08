# \ImagesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_image_upload**](ImagesApi.md#complete_image_upload) | **POST** /images/upload/complete | Upload: Mark as complete
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /images/{imageId} | Delete image
[**get_image**](ImagesApi.md#get_image) | **GET** /images/{imageId} | Get image
[**init_image_upload**](ImagesApi.md#init_image_upload) | **POST** /images/upload/init | Upload: Init
[**list_images**](ImagesApi.md#list_images) | **GET** /images | List images for a project



## complete_image_upload

> models::Image complete_image_upload(image_upload_complete_request)
Upload: Mark as complete

Complete the upload process and create the image record using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_upload_complete_request** | [**ImageUploadCompleteRequest**](ImageUploadCompleteRequest.md) | Upload completion request | [required] |

### Return type

[**models::Image**](Image.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> delete_image(image_id)
Delete image

delete an image by marking it as deleted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | Image ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::Image get_image(image_id)
Get image

Retrieve an image by its ID for a specific project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | Image ID | [required] |

### Return type

[**models::Image**](Image.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## init_image_upload

> models::ImageUploadInitResponse init_image_upload(image_upload_init_request)
Upload: Init

Initialize a presigned URL upload for an image file using API key authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_upload_init_request** | [**ImageUploadInitRequest**](ImageUploadInitRequest.md) | Upload initialization request | [required] |

### Return type

[**models::ImageUploadInitResponse**](internal_images_handler.initResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images

> models::PaginationPaginatedResponseImage list_images(limit, offset, sort, order)
List images for a project

Retrieve all images for a specific project, with pagination and sorting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of items to return in a single request. <br> **Default:** `25` |  |[default to 25]
**offset** | Option<**i32**> | Starting point of the result set. <br>To get page 2 with a limit of 25, set `offset` to `25`. <br> **Default:** `0` |  |[default to 0]
**sort** | Option<**String**> | Field to sort by (created_at, name, size, updated_at) |  |
**order** | Option<**String**> | Sort order (asc, desc) |  |

### Return type

[**models::PaginationPaginatedResponseImage**](pagination.PaginatedResponse-Image.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

