# \SessionsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logout**](SessionsApi.md#logout) | **POST** /auth/v1/logout | Logout
[**refresh_token**](SessionsApi.md#refresh_token) | **POST** /auth/v1/token | RefreshToken



## logout

> serde_json::Value logout(auth_v1_logout_request)
Logout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_logout_request** | [**AuthV1LogoutRequest**](AuthV1LogoutRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> models::AuthV1TokenResponse refresh_token(auth_v1_refresh_token_request)
RefreshToken

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_refresh_token_request** | [**AuthV1RefreshTokenRequest**](AuthV1RefreshTokenRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

