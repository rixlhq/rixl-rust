# \UsersApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_avatar_upload**](UsersApi.md#create_avatar_upload) | **POST** /auth/v1/users/current/avatar/upload | CreateAvatarUpload
[**get_user**](UsersApi.md#get_user) | **GET** /auth/v1/users/current | GetUser
[**get_user_info**](UsersApi.md#get_user_info) | **GET** /auth/v1/userinfo | GetUserInfo
[**update_name**](UsersApi.md#update_name) | **PATCH** /auth/v1/users/current/name | UpdateName
[**update_user_avatar**](UsersApi.md#update_user_avatar) | **PATCH** /auth/v1/users/current/avatar | UpdateUserAvatar
[**update_username**](UsersApi.md#update_username) | **PATCH** /auth/v1/users/current/username | UpdateUsername



## create_avatar_upload

> models::AuthV1CreateAvatarUploadResponse create_avatar_upload(user_id)
CreateAvatarUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1CreateAvatarUploadResponse**](auth.v1.CreateAvatarUploadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::AuthV1GetUserResponse get_user(user_id)
GetUser

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1GetUserResponse**](auth.v1.GetUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_info

> models::AuthV1UserInfo get_user_info(user_id)
GetUserInfo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1UserInfo**](auth.v1.UserInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_name

> models::AuthV1UpdateNameResponse update_name(auth_v1_update_name_request)
UpdateName

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_update_name_request** | [**AuthV1UpdateNameRequest**](AuthV1UpdateNameRequest.md) |  | [required] |

### Return type

[**models::AuthV1UpdateNameResponse**](auth.v1.UpdateNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_avatar

> models::AuthV1UpdateUserAvatarResponse update_user_avatar(auth_v1_update_user_avatar_request)
UpdateUserAvatar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_update_user_avatar_request** | [**AuthV1UpdateUserAvatarRequest**](AuthV1UpdateUserAvatarRequest.md) |  | [required] |

### Return type

[**models::AuthV1UpdateUserAvatarResponse**](auth.v1.UpdateUserAvatarResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_username

> models::AuthV1UpdateUsernameResponse update_username(auth_v1_update_username_request)
UpdateUsername

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_update_username_request** | [**AuthV1UpdateUsernameRequest**](AuthV1UpdateUsernameRequest.md) |  | [required] |

### Return type

[**models::AuthV1UpdateUsernameResponse**](auth.v1.UpdateUsernameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

