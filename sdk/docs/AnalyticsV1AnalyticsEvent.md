# AnalyticsV1AnalyticsEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**content_view** | Option<[**models::AnalyticsV1ContentViewEvent**](AnalyticsV1ContentViewEvent.md)> |  | [optional]
**engagement** | Option<[**models::AnalyticsV1EngagementEvent**](AnalyticsV1EngagementEvent.md)> |  | [optional]
**error** | Option<[**models::AnalyticsV1ErrorEvent**](AnalyticsV1ErrorEvent.md)> |  | [optional]
**interaction** | Option<[**models::AnalyticsV1InteractionEvent**](AnalyticsV1InteractionEvent.md)> |  | [optional]
**session_start** | Option<[**models::AnalyticsV1SessionStartEvent**](AnalyticsV1SessionStartEvent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


