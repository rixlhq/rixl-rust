# \SignInsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_sign_ins**](SignInsApi.md#list_sign_ins) | **GET** /auth/v1/sign-ins | ListSignIns
[**logout**](SignInsApi.md#logout) | **POST** /auth/v1/logout | Logout
[**logout_all**](SignInsApi.md#logout_all) | **POST** /auth/v1/logout/all | LogoutAll
[**refresh_token**](SignInsApi.md#refresh_token) | **POST** /auth/v1/token | RefreshToken
[**revoke_other_sign_ins**](SignInsApi.md#revoke_other_sign_ins) | **POST** /auth/v1/sign-ins/revoke-others | RevokeOtherSignIns
[**revoke_sign_in**](SignInsApi.md#revoke_sign_in) | **DELETE** /auth/v1/sign-ins/{sign_in_id} | RevokeSignIn



## list_sign_ins

> models::AuthV1ListSignInsResponse list_sign_ins(user_id)
ListSignIns

Lists the caller's sign-ins, most recently used first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |

### Return type

[**models::AuthV1ListSignInsResponse**](auth.v1.ListSignInsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## logout_all

> serde_json::Value logout_all(user_id)
LogoutAll

Revokes all of the caller's sign-ins and invalidates their tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
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


## revoke_other_sign_ins

> serde_json::Value revoke_other_sign_ins(user_id)
RevokeOtherSignIns

Revokes all of the caller's sign-ins except the current one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_sign_in

> serde_json::Value revoke_sign_in(sign_in_id, user_user_id)
RevokeSignIn

Revokes one of the caller's sign-ins and invalidates its tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_in_id** | **String** |  | [required] |
**user_user_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

