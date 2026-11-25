# AuthV1SignIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**last_used_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**auth_method** | Option<**String**> | auth_method is how the sign-in was started: \"password\", \"passkey\",  \"passkey_2fa\", \"totp\", or \"provider:<name>\". | [optional]
**user_agent** | Option<**String**> |  | [optional]
**ip** | Option<**String**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**current** | Option<**bool**> | current marks the sign-in of the access token making the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


