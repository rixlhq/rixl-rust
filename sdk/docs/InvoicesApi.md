# \InvoicesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_invoices**](InvoicesApi.md#list_invoices) | **GET** /billing/v1/invoices | ListInvoices
[**update_invoice_status**](InvoicesApi.md#update_invoice_status) | **PATCH** /billing/v1/invoices/{invoice_id}/status | UpdateInvoiceStatus



## list_invoices

> models::BillingV1ListInvoicesResponse list_invoices(org_id, pagination_limit, pagination_offset)
ListInvoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::BillingV1ListInvoicesResponse**](billing.v1.ListInvoicesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_invoice_status

> models::BillingV1Invoice update_invoice_status(invoice_id, update_invoice_status_request)
UpdateInvoiceStatus

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The invoice_id path parameter. | [required] |
**update_invoice_status_request** | [**UpdateInvoiceStatusRequest**](UpdateInvoiceStatusRequest.md) |  | [required] |

### Return type

[**models::BillingV1Invoice**](billing.v1.Invoice.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

