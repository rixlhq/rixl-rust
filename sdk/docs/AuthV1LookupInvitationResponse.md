# AuthV1LookupInvitationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<[**models::AuthV1InvitationState**](AuthV1InvitationState.md)> |  | [optional]
**org_name** | Option<**String**> |  | [optional]
**org_image_id** | Option<**String**> |  | [optional]
**role** | Option<[**models::AuthV1MembershipRole**](AuthV1MembershipRole.md)> |  | [optional]
**inviter_display_name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


