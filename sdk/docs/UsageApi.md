# \UsageApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bandwidth_usage**](UsageApi.md#get_bandwidth_usage) | **GET** /billing/v1/bandwidth-usage | GetBandwidthUsage
[**get_bandwidth_usage_history**](UsageApi.md#get_bandwidth_usage_history) | **GET** /billing/v1/bandwidth-usage/history | GetBandwidthUsageHistory
[**get_storage_usage**](UsageApi.md#get_storage_usage) | **GET** /billing/v1/storage-usage | GetStorageUsage
[**get_storage_usage_history**](UsageApi.md#get_storage_usage_history) | **GET** /billing/v1/storage-usage/history | GetStorageUsageHistory
[**refresh_bandwidth_usage**](UsageApi.md#refresh_bandwidth_usage) | **POST** /billing/v1/bandwidth-usage/refresh | RefreshBandwidthUsage
[**refresh_storage_usage**](UsageApi.md#refresh_storage_usage) | **POST** /billing/v1/storage-usage/refresh | RefreshStorageUsage



## get_bandwidth_usage

> models::BillingV1BandwidthUsage get_bandwidth_usage(org_id)
GetBandwidthUsage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1BandwidthUsage**](billing.v1.BandwidthUsage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bandwidth_usage_history

> models::BillingV1GetBandwidthUsageHistoryResponse get_bandwidth_usage_history(org_id, days)
GetBandwidthUsageHistory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**days** | Option<**i32**> | The days query parameter. |  |

### Return type

[**models::BillingV1GetBandwidthUsageHistoryResponse**](billing.v1.GetBandwidthUsageHistoryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_usage

> models::BillingV1StorageUsage get_storage_usage(org_id)
GetStorageUsage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1StorageUsage**](billing.v1.StorageUsage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_usage_history

> models::BillingV1GetStorageUsageHistoryResponse get_storage_usage_history(org_id, days)
GetStorageUsageHistory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**days** | Option<**i32**> | The days query parameter. |  |

### Return type

[**models::BillingV1GetStorageUsageHistoryResponse**](billing.v1.GetStorageUsageHistoryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_bandwidth_usage

> models::BillingV1BandwidthUsage refresh_bandwidth_usage(org_id)
RefreshBandwidthUsage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1BandwidthUsage**](billing.v1.BandwidthUsage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_storage_usage

> models::BillingV1StorageUsage refresh_storage_usage(org_id)
RefreshStorageUsage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1StorageUsage**](billing.v1.StorageUsage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

