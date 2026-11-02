# WebhooksV1WebhookAttempt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**event_id** | Option<**String**> |  | [optional]
**attempt** | Option<**i32**> |  | [optional]
**replay** | Option<**bool**> | True when this delivery was triggered by ReplayEvent or SendTestEvent  rather than the original subscription fan-out. | [optional]
**status_code** | Option<**i32**> |  | [optional]
**error** | Option<**String**> |  | [optional]
**duration_ms** | Option<**i32**> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**response_snippet** | Option<**String**> | First 1 KiB of the endpoint's response body. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


