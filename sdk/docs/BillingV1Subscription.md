# BillingV1Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**org_id** | Option<**String**> |  | [optional]
**plan_id** | Option<**String**> |  | [optional]
**plan_name** | Option<**String**> |  | [optional]
**plan_type** | Option<[**models::BillingV1PlanType**](BillingV1PlanType.md)> |  | [optional]
**status** | Option<[**models::BillingV1SubscriptionStatus**](BillingV1SubscriptionStatus.md)> |  | [optional]
**current_period_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**cancel_at_period_end** | Option<**bool**> |  | [optional]
**price** | Option<**String**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**expiring_soon** | Option<**bool**> |  | [optional]
**trials_ending_soon** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


