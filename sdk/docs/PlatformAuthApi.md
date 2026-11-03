# \PlatformAuthApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_api_key**](PlatformAuthApi.md#exchange_api_key) | **POST** /platform/auth/v1/token | ExchangeAPIKey
[**refresh_platform_token**](PlatformAuthApi.md#refresh_platform_token) | **POST** /platform/auth/v1/refresh | RefreshPlatformToken



## exchange_api_key

> models::PlatformauthV1TokenResponse exchange_api_key(platformauth_v1_exchange_api_key_request)
ExchangeAPIKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platformauth_v1_exchange_api_key_request** | [**PlatformauthV1ExchangeApiKeyRequest**](PlatformauthV1ExchangeApiKeyRequest.md) |  | [required] |

### Return type

[**models::PlatformauthV1TokenResponse**](platformauth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_platform_token

> models::PlatformauthV1TokenResponse refresh_platform_token(platformauth_v1_refresh_platform_token_request)
RefreshPlatformToken

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platformauth_v1_refresh_platform_token_request** | [**PlatformauthV1RefreshPlatformTokenRequest**](PlatformauthV1RefreshPlatformTokenRequest.md) |  | [required] |

### Return type

[**models::PlatformauthV1TokenResponse**](platformauth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

