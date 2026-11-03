# \SalesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contact_sales**](SalesApi.md#contact_sales) | **POST** /billing/v1/contact-sales | ContactSales



## contact_sales

> models::BillingV1SalesLead contact_sales(billing_v1_contact_sales_request)
ContactSales

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_contact_sales_request** | [**BillingV1ContactSalesRequest**](BillingV1ContactSalesRequest.md) |  | [required] |

### Return type

[**models::BillingV1SalesLead**](billing.v1.SalesLead.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

