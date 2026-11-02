# BillingV1CreateSubscriptionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**plan_id** | Option<**String**> |  | [optional]
**status** | Option<[**models::BillingV1SubscriptionStatus**](BillingV1SubscriptionStatus.md)> |  | [optional]
**current_period_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**cancel_at_period_end** | Option<**bool**> |  | [optional]
**stripe_subscription_id** | Option<**String**> |  | [optional]
**stripe_customer_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


