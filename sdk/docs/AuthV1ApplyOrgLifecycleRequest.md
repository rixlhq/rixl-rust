# AuthV1ApplyOrgLifecycleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**org_id** | Option<**String**> |  | [optional]
**state** | Option<[**models::AuthV1OrganizationState**](AuthV1OrganizationState.md)> |  | [optional]
**lifecycle_version** | Option<**String**> |  | [optional]
**purge_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**reservation_id** | Option<**String**> | Set for api when the transition was prepared by billing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


