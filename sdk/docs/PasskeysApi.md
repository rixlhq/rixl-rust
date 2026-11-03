# \PasskeysApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_passkey**](PasskeysApi.md#delete_passkey) | **DELETE** /auth/v1/users/current/passkeys/{id} | DeletePasskey
[**list_passkeys**](PasskeysApi.md#list_passkeys) | **GET** /auth/v1/users/current/passkeys | ListPasskeys
[**passkey_login_begin**](PasskeysApi.md#passkey_login_begin) | **POST** /auth/v1/passkey/login/begin | PasskeyLoginBegin
[**passkey_login_finish**](PasskeysApi.md#passkey_login_finish) | **POST** /auth/v1/passkey/login/finish | PasskeyLoginFinish
[**passkey_register_begin**](PasskeysApi.md#passkey_register_begin) | **POST** /auth/v1/users/current/passkeys/register/begin | PasskeyRegisterBegin
[**passkey_register_finish**](PasskeysApi.md#passkey_register_finish) | **POST** /auth/v1/users/current/passkeys/register/finish | PasskeyRegisterFinish
[**rename_passkey**](PasskeysApi.md#rename_passkey) | **PATCH** /auth/v1/users/current/passkeys/{id} | RenamePasskey
[**verify_passkey_for_login**](PasskeysApi.md#verify_passkey_for_login) | **POST** /auth/v1/verify-passkey | VerifyPasskeyForLogin



## delete_passkey

> serde_json::Value delete_passkey(id, user_id)
DeletePasskey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_passkeys

> models::AuthV1ListPasskeysResponse list_passkeys(user_id)
ListPasskeys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1ListPasskeysResponse**](auth.v1.ListPasskeysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## passkey_login_begin

> models::AuthV1PasskeyBeginResponse passkey_login_begin(body)
PasskeyLoginBegin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::AuthV1PasskeyBeginResponse**](auth.v1.PasskeyBeginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## passkey_login_finish

> models::AuthV1TokenResponse passkey_login_finish(auth_v1_passkey_login_finish_request)
PasskeyLoginFinish

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_passkey_login_finish_request** | [**AuthV1PasskeyLoginFinishRequest**](AuthV1PasskeyLoginFinishRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## passkey_register_begin

> models::AuthV1PasskeyBeginResponse passkey_register_begin(auth_v1_passkey_register_begin_request)
PasskeyRegisterBegin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_passkey_register_begin_request** | [**AuthV1PasskeyRegisterBeginRequest**](AuthV1PasskeyRegisterBeginRequest.md) |  | [required] |

### Return type

[**models::AuthV1PasskeyBeginResponse**](auth.v1.PasskeyBeginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## passkey_register_finish

> models::AuthV1PasskeyRegisterFinishResponse passkey_register_finish(auth_v1_passkey_register_finish_request)
PasskeyRegisterFinish

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_passkey_register_finish_request** | [**AuthV1PasskeyRegisterFinishRequest**](AuthV1PasskeyRegisterFinishRequest.md) |  | [required] |

### Return type

[**models::AuthV1PasskeyRegisterFinishResponse**](auth.v1.PasskeyRegisterFinishResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_passkey

> serde_json::Value rename_passkey(id, rename_passkey_request)
RenamePasskey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**rename_passkey_request** | [**RenamePasskeyRequest**](RenamePasskeyRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_passkey_for_login

> models::AuthV1TokenResponse verify_passkey_for_login(auth_v1_verify_passkey_for_login_request)
VerifyPasskeyForLogin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_verify_passkey_for_login_request** | [**AuthV1VerifyPasskeyForLoginRequest**](AuthV1VerifyPasskeyForLoginRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

