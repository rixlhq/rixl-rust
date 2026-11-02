# BillingV1CalculateTaxRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**org_id** | Option<**String**> |  | [optional]
**plan_id** | **String** |  | 
**plan_name** | **String** |  | 
**amount** | Option<**f64**> |  | [optional]
**currency** | **String** |  | 
**billing_cycle** | Option<[**models::BillingV1BillingCycle**](BillingV1BillingCycle.md)> | enum.defined_only = true  | [optional]
**interval_count** | Option<**String**> |  | [optional]
**billing_address** | Option<[**models::BillingV1BillingAddress**](BillingV1BillingAddress.md)> |  | [optional]
**line_items** | Option<[**Vec<models::BillingV1TaxLineItem>**](BillingV1TaxLineItem.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


