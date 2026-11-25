# AuthV1NotificationPreference

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | Option<[**models::AuthV1NotificationCategory**](AuthV1NotificationCategory.md)> |  | [optional]
**email_enabled** | Option<**bool**> |  | [optional]
**locked** | Option<**bool**> | locked is true when the caller's role cannot disable the category. | [optional]
**always_sent** | Option<**Vec<String>**> | always_sent lists the notices in this category the caller receives even with email disabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


