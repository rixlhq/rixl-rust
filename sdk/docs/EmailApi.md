# \EmailApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_email**](EmailApi.md#add_email) | **POST** /auth/v1/users/current/emails | AddEmail
[**get_user_email_status**](EmailApi.md#get_user_email_status) | **GET** /auth/v1/users/current/emails/status | GetUserEmailStatus
[**initiate_email_change**](EmailApi.md#initiate_email_change) | **PUT** /auth/v1/users/current/emails/change | InitiateEmailChange
[**login**](EmailApi.md#login) | **POST** /auth/v1/login | Login
[**register**](EmailApi.md#register) | **POST** /auth/v1/register | Register
[**resend_verification**](EmailApi.md#resend_verification) | **POST** /auth/v1/email/verify/resend | ResendVerification
[**reset_password**](EmailApi.md#reset_password) | **POST** /auth/v1/password/reset/confirm | ResetPassword
[**send_password_reset**](EmailApi.md#send_password_reset) | **POST** /auth/v1/password/reset | SendPasswordReset
[**verify_email**](EmailApi.md#verify_email) | **POST** /auth/v1/email/verify | VerifyEmail



## add_email

> models::AuthV1VerificationSentResponse add_email(auth_v1_add_email_request)
AddEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_add_email_request** | [**AuthV1AddEmailRequest**](AuthV1AddEmailRequest.md) |  | [required] |

### Return type

[**models::AuthV1VerificationSentResponse**](auth.v1.VerificationSentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_email_status

> models::AuthV1EmailStatusResponse get_user_email_status(user_id)
GetUserEmailStatus

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1EmailStatusResponse**](auth.v1.EmailStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_email_change

> models::AuthV1VerificationSentResponse initiate_email_change(auth_v1_initiate_email_change_request)
InitiateEmailChange

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_initiate_email_change_request** | [**AuthV1InitiateEmailChangeRequest**](AuthV1InitiateEmailChangeRequest.md) |  | [required] |

### Return type

[**models::AuthV1VerificationSentResponse**](auth.v1.VerificationSentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> models::AuthV1LoginResponse login(auth_v1_login_request)
Login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_login_request** | [**AuthV1LoginRequest**](AuthV1LoginRequest.md) |  | [required] |

### Return type

[**models::AuthV1LoginResponse**](auth.v1.LoginResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::AuthV1RegisterResponse register(auth_v1_register_request)
Register

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_register_request** | [**AuthV1RegisterRequest**](AuthV1RegisterRequest.md) |  | [required] |

### Return type

[**models::AuthV1RegisterResponse**](auth.v1.RegisterResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_verification

> models::AuthV1ResendVerificationResponse resend_verification(auth_v1_resend_verification_request)
ResendVerification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_resend_verification_request** | [**AuthV1ResendVerificationRequest**](AuthV1ResendVerificationRequest.md) |  | [required] |

### Return type

[**models::AuthV1ResendVerificationResponse**](auth.v1.ResendVerificationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> serde_json::Value reset_password(auth_v1_reset_password_request)
ResetPassword

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_reset_password_request** | [**AuthV1ResetPasswordRequest**](AuthV1ResetPasswordRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_password_reset

> serde_json::Value send_password_reset(auth_v1_send_password_reset_request)
SendPasswordReset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_send_password_reset_request** | [**AuthV1SendPasswordResetRequest**](AuthV1SendPasswordResetRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_email

> models::AuthV1VerifyEmailResponse verify_email(auth_v1_verify_email_request)
VerifyEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_verify_email_request** | [**AuthV1VerifyEmailRequest**](AuthV1VerifyEmailRequest.md) |  | [required] |

### Return type

[**models::AuthV1VerifyEmailResponse**](auth.v1.VerifyEmailResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

