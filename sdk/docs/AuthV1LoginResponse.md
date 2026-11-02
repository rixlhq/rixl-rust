# AuthV1LoginResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | \"ok\" | \"2fa_required\" | \"email_not_verified\" | [optional]
**session_id** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**access_token** | Option<**String**> |  | [optional]
**refresh_token** | Option<**String**> |  | [optional]
**expires_in** | Option<**String**> |  | [optional]
**token_type** | Option<**String**> |  | [optional]
**requires_action** | Option<**String**> |  | [optional]
**authentication** | Option<[**Vec<models::AuthV1AuthMethod>**](AuthV1AuthMethod.md)> | authentication lists the 2FA methods the user has configured when  status is \"2fa_required\". The public API renders these as lowercase  strings: \"passkey\", \"totp\". | [optional]
**passkey_options** | Option<**String**> | passkey_options is the WebAuthn PublicKeyCredentialRequestOptions as JSON,  present only when \"passkey\" is one of the authentication methods. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


