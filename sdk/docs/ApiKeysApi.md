# \ApiKeysApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /organizations/{org_id}/api-keys/v1 | CreateApiKey
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /organizations/{org_id}/api-keys/v1/{key_id} | DeleteApiKey
[**list_api_keys**](ApiKeysApi.md#list_api_keys) | **GET** /organizations/{org_id}/api-keys/v1 | ListApiKeys
[**rotate_api_key**](ApiKeysApi.md#rotate_api_key) | **POST** /organizations/{org_id}/api-keys/v1/{key_id}/rotate | RotateApiKey
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PATCH** /organizations/{org_id}/api-keys/v1/{key_id} | UpdateApiKey



## create_api_key

> models::ApikeysV1CreateApiKeyResponse create_api_key(org_id, create_api_key_request)
CreateApiKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::ApikeysV1CreateApiKeyResponse**](apikeys.v1.CreateApiKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> serde_json::Value delete_api_key(org_id, key_id)
DeleteApiKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**key_id** | **String** | The key_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> models::ApikeysV1ListApiKeysResponse list_api_keys(org_id, pagination_limit, pagination_offset)
ListApiKeys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::ApikeysV1ListApiKeysResponse**](apikeys.v1.ListApiKeysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_api_key

> models::ApikeysV1RotateApiKeyResponse rotate_api_key(org_id, key_id)
RotateApiKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**key_id** | **String** | The key_id path parameter. | [required] |

### Return type

[**models::ApikeysV1RotateApiKeyResponse**](apikeys.v1.RotateApiKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> models::ApikeysV1UpdateApiKeyResponse update_api_key(org_id, key_id, update_api_key_request)
UpdateApiKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**key_id** | **String** | The key_id path parameter. | [required] |
**update_api_key_request** | [**UpdateApiKeyRequest**](UpdateApiKeyRequest.md) |  | [required] |

### Return type

[**models::ApikeysV1UpdateApiKeyResponse**](apikeys.v1.UpdateApiKeyResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

