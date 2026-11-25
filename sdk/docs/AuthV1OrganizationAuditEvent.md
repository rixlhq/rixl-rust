# AuthV1OrganizationAuditEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**occurred_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**actor_type** | Option<**String**> |  | [optional]
**actor_id** | Option<**String**> |  | [optional]
**action** | Option<**String**> |  | [optional]
**target_type** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | A JSON object whose property values may be any JSON value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


