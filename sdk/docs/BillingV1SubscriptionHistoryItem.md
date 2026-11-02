# BillingV1SubscriptionHistoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**plan_id** | Option<**String**> |  | [optional]
**plan_name** | Option<**String**> |  | [optional]
**status** | Option<[**models::BillingV1SubscriptionStatus**](BillingV1SubscriptionStatus.md)> |  | [optional]
**price** | Option<**String**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**current_period_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**cancel_at_period_end** | Option<**bool**> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


