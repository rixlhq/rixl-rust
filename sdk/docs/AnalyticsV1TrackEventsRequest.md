# AnalyticsV1TrackEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**events** | Option<[**Vec<models::AnalyticsV1AnalyticsEvent>**](AnalyticsV1AnalyticsEvent.md)> |  | [optional]
**city** | Option<**String**> |  | [optional]
**region** | Option<**String**> |  | [optional]
**device** | Option<**String**> |  | [optional]
**os** | Option<**String**> |  | [optional]
**os_version** | Option<**String**> |  | [optional]
**language** | Option<**String**> |  | [optional]
**browser** | Option<**String**> |  | [optional]
**project_id** | Option<**String**> | Project the events belong to (Organization > Project > Videos/Images/Posts/Feeds).  Sent alongside the other envelope-level context because a client session is scoped  to one project. org_id is still derived server-side from the credential and is  never taken from the body, so a wrong project_id can only mis-attribute within the  caller's own organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


