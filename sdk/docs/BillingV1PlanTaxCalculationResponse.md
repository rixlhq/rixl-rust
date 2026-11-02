# BillingV1PlanTaxCalculationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**calculation_id** | Option<**String**> |  | [optional]
**plan_id** | Option<**String**> |  | [optional]
**plan_name** | Option<**String**> |  | [optional]
**billing_cycle** | Option<[**models::BillingV1BillingCycle**](BillingV1BillingCycle.md)> |  | [optional]
**base_amount** | Option<**String**> |  | [optional]
**amount_total** | Option<**String**> |  | [optional]
**tax_amount_exclusive** | Option<**String**> |  | [optional]
**tax_amount_inclusive** | Option<**String**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**tax_percentage** | Option<**f64**> |  | [optional]
**billing_address** | Option<[**models::BillingV1BillingAddress**](BillingV1BillingAddress.md)> |  | [optional]
**calculated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


