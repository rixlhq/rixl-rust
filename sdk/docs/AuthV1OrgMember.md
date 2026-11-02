# AuthV1OrgMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**role** | Option<[**models::AuthV1MembershipRole**](AuthV1MembershipRole.md)> |  | [optional]
**state** | Option<[**models::AuthV1MembershipState**](AuthV1MembershipState.md)> |  | [optional]
**username** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**invitation_expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**joined_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


