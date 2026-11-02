# \ImagesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_image_upload**](ImagesApi.md#create_image_upload) | **POST** /media/v1/projects/{project_id}/images/upload | CreateImageUpload
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /media/v1/projects/{project_id}/images/{image_id} | DeleteImage
[**get_image**](ImagesApi.md#get_image) | **GET** /media/v1/images/{image_id} | GetImage
[**list_images**](ImagesApi.md#list_images) | **GET** /media/v1/projects/{project_id}/images | ListImages
[**update_image_visibility**](ImagesApi.md#update_image_visibility) | **PATCH** /media/v1/projects/{project_id}/images/{image_id}/visibility | UpdateImageVisibility



## create_image_upload

> models::ImagesV1ImageUpload create_image_upload(project_id, create_image_upload_request)
CreateImageUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**create_image_upload_request** | [**CreateImageUploadRequest**](CreateImageUploadRequest.md) |  | [required] |

### Return type

[**models::ImagesV1ImageUpload**](images.v1.ImageUpload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> serde_json::Value delete_image(project_id, image_id)
DeleteImage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**image_id** | **String** | The image_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::ImagesV1GetImageResponse get_image(image_id)
GetImage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | The image_id path parameter. | [required] |

### Return type

[**models::ImagesV1GetImageResponse**](images.v1.GetImageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images

> models::ImagesV1ListImagesResponse list_images(project_id, pagination_limit, pagination_offset, sort_field, sort_direction)
ListImages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]
**sort_field** | Option<**String**> | The sort_field query parameter. |  |
**sort_direction** | Option<**String**> | The sort_direction query parameter. |  |

### Return type

[**models::ImagesV1ListImagesResponse**](images.v1.ListImagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_image_visibility

> models::ImagesV1UpdateImageVisibilityResponse update_image_visibility(project_id, image_id, update_image_visibility_request)
UpdateImageVisibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project_id path parameter. | [required] |
**image_id** | **String** | The image_id path parameter. | [required] |
**update_image_visibility_request** | [**UpdateImageVisibilityRequest**](UpdateImageVisibilityRequest.md) |  | [required] |

### Return type

[**models::ImagesV1UpdateImageVisibilityResponse**](images.v1.UpdateImageVisibilityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

