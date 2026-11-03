# \SocialProvidersApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_provider**](SocialProvidersApi.md#connect_provider) | **POST** /auth/v1/providers/connect | ConnectProvider
[**disconnect_provider**](SocialProvidersApi.md#disconnect_provider) | **DELETE** /auth/v1/providers/{provider} | DisconnectProvider
[**list_providers**](SocialProvidersApi.md#list_providers) | **GET** /auth/v1/providers | ListProviders



## connect_provider

> models::AuthV1TokenResponse connect_provider(auth_v1_connect_provider_request)
ConnectProvider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_connect_provider_request** | [**AuthV1ConnectProviderRequest**](AuthV1ConnectProviderRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_provider

> serde_json::Value disconnect_provider(provider, user_user_id)
DisconnectProvider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | **String** | The provider path parameter. | [required] |
**user_user_id** | Option<**String**> | The user.user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_providers

> models::AuthV1ListProvidersResponse list_providers(user_id)
ListProviders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1ListProvidersResponse**](auth.v1.ListProvidersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

