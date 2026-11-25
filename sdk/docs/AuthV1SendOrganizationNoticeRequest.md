# AuthV1SendOrganizationNoticeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**org_id** | Option<**String**> |  | [optional]
**category** | Option<[**models::AuthV1NotificationCategory**](AuthV1NotificationCategory.md)> |  | [optional]
**template** | Option<**String**> |  | [optional]
**data** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**idempotency_key** | Option<**String**> |  | [optional]
**owners_mandatory** | Option<**bool**> | owners_mandatory is ignored: auth-api decides which notices owners always receive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


