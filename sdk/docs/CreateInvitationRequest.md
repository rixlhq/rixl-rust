# CreateInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | Trimmed and lowercased before validation as a bare email address. The normalized address must be nonempty and at most 254 UTF-8 bytes, with a local part of at most 64 bytes and a domain containing a dot that is neither leading nor trailing. | [optional]
**role** | Option<[**models::AuthV1MembershipRole**](AuthV1MembershipRole.md)> |  | [optional]
**request_id** | Option<**String**> | Optional UUID that makes retries of the same create safe. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


