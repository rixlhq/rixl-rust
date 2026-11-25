# AuthV1Invitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**org** | Option<[**models::AuthV1Organization**](AuthV1Organization.md)> |  | [optional]
**email** | Option<**String**> |  | [optional]
**role** | Option<[**models::AuthV1MembershipRole**](AuthV1MembershipRole.md)> | Never OWNER. | [optional]
**state** | Option<[**models::AuthV1InvitationState**](AuthV1InvitationState.md)> | The effective state: a pending invitation past its expiry reads as expired. | [optional]
**invited_by** | Option<[**models::AuthV1InvitationInviter**](AuthV1InvitationInviter.md)> | Unset once the inviter's account is deleted. | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**last_sent_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**resend_available_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Set while a daily sending limit would delay a resend by the caller. | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


