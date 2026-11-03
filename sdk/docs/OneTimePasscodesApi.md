# \OneTimePasscodesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_otp**](OneTimePasscodesApi.md#delete_otp) | **DELETE** /auth/v1/users/current/totp/delete | DeleteOTP
[**get_otp_status**](OneTimePasscodesApi.md#get_otp_status) | **GET** /auth/v1/users/current/totp/status | GetOTPStatus
[**regenerate_backup_codes**](OneTimePasscodesApi.md#regenerate_backup_codes) | **POST** /auth/v1/users/current/totp/backup-codes/regenerate | RegenerateBackupCodes
[**setup_otp**](OneTimePasscodesApi.md#setup_otp) | **POST** /auth/v1/users/current/totp/setup | SetupOTP
[**verify_otp**](OneTimePasscodesApi.md#verify_otp) | **POST** /auth/v1/users/current/totp/verify | VerifyOTP
[**verify_totp_for_login**](OneTimePasscodesApi.md#verify_totp_for_login) | **POST** /auth/v1/verify-totp | VerifyTOTPForLogin



## delete_otp

> serde_json::Value delete_otp(user_id)
DeleteOTP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_otp_status

> models::AuthV1OtpStatusResponse get_otp_status(user_id)
GetOTPStatus

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1OtpStatusResponse**](auth.v1.OTPStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_backup_codes

> models::AuthV1BackupCodesResponse regenerate_backup_codes(user_id)
RegenerateBackupCodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1BackupCodesResponse**](auth.v1.BackupCodesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## setup_otp

> models::AuthV1SetupOtpResponse setup_otp(user_id)
SetupOTP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1SetupOtpResponse**](auth.v1.SetupOTPResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_otp

> models::AuthV1TokenResponse verify_otp(auth_v1_verify_otp_request)
VerifyOTP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_verify_otp_request** | [**AuthV1VerifyOtpRequest**](AuthV1VerifyOtpRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_totp_for_login

> models::AuthV1TokenResponse verify_totp_for_login(auth_v1_verify_totp_for_login_request)
VerifyTOTPForLogin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_verify_totp_for_login_request** | [**AuthV1VerifyTotpForLoginRequest**](AuthV1VerifyTotpForLoginRequest.md) |  | [required] |

### Return type

[**models::AuthV1TokenResponse**](auth.v1.TokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

