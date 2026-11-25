# \NotificationPreferencesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_notification_preferences**](NotificationPreferencesApi.md#get_notification_preferences) | **GET** /auth/v1/notifications/{org_id}/preferences | GetNotificationPreferences
[**update_notification_preference**](NotificationPreferencesApi.md#update_notification_preference) | **PATCH** /auth/v1/notifications/{user_org_id}/preferences | UpdateNotificationPreference



## get_notification_preferences

> models::AuthV1NotificationPreferences get_notification_preferences(org_id, user_id)
GetNotificationPreferences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::AuthV1NotificationPreferences**](auth.v1.NotificationPreferences.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_preference

> models::AuthV1NotificationPreference update_notification_preference(user_org_id, update_notification_preference_request)
UpdateNotificationPreference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**update_notification_preference_request** | [**UpdateNotificationPreferenceRequest**](UpdateNotificationPreferenceRequest.md) |  | [required] |

### Return type

[**models::AuthV1NotificationPreference**](auth.v1.NotificationPreference.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

