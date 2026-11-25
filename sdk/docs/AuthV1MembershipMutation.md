# AuthV1MembershipMutation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**role** | Option<[**models::AuthV1MembershipRole**](AuthV1MembershipRole.md)> |  | [optional]
**state** | Option<[**models::AuthV1MembershipState**](AuthV1MembershipState.md)> |  | [optional]
**joined_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**organization** | Option<[**models::AuthV1Organization**](AuthV1Organization.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


