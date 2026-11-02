# \SubscriptionsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_subscription**](SubscriptionsApi.md#cancel_subscription) | **POST** /billing/v1/subscription/cancel | CancelSubscription
[**create_subscription**](SubscriptionsApi.md#create_subscription) | **POST** /billing/v1/subscription | CreateSubscription
[**get_subscription**](SubscriptionsApi.md#get_subscription) | **GET** /billing/v1/subscription | GetSubscription
[**get_subscription_history**](SubscriptionsApi.md#get_subscription_history) | **GET** /billing/v1/subscription/history | GetSubscriptionHistory
[**reactivate_subscription**](SubscriptionsApi.md#reactivate_subscription) | **POST** /billing/v1/subscription/reactivate | ReactivateSubscription



## cancel_subscription

> serde_json::Value cancel_subscription(org_id)
CancelSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> models::BillingV1CreateSubscriptionResponse create_subscription(billing_v1_create_subscription_request)
CreateSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_create_subscription_request** | [**BillingV1CreateSubscriptionRequest**](BillingV1CreateSubscriptionRequest.md) |  | [required] |

### Return type

[**models::BillingV1CreateSubscriptionResponse**](billing.v1.CreateSubscriptionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription

> models::BillingV1Subscription get_subscription(org_id)
GetSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1Subscription**](billing.v1.Subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_history

> models::BillingV1GetSubscriptionHistoryResponse get_subscription_history(org_id, pagination_limit, pagination_offset)
GetSubscriptionHistory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::BillingV1GetSubscriptionHistoryResponse**](billing.v1.GetSubscriptionHistoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactivate_subscription

> serde_json::Value reactivate_subscription(org_id)
ReactivateSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

