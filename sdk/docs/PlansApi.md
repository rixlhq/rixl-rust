# \PlansApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plan**](PlansApi.md#get_plan) | **GET** /billing/v1/plans/{plan_id} | GetPlan
[**list_plans**](PlansApi.md#list_plans) | **GET** /billing/v1/plans | ListPlans



## get_plan

> models::BillingV1GetPlanResponse get_plan(plan_id)
GetPlan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** | string.prefix = \"price_\"  | [required] |

### Return type

[**models::BillingV1GetPlanResponse**](billing.v1.GetPlanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_plans

> models::BillingV1ListPlansResponse list_plans(currency, interval, free)
ListPlans

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<**String**> | The currency query parameter. |  |
**interval** | Option<**String**> | The interval query parameter. |  |
**free** | Option<**String**> | The free query parameter. |  |

### Return type

[**models::BillingV1ListPlansResponse**](billing.v1.ListPlansResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

