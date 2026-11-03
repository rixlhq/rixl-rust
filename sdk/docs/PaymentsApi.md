# \PaymentsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calculate_generic_tax**](PaymentsApi.md#calculate_generic_tax) | **POST** /billing/v1/tax/calculate-generic | CalculateGenericTax
[**calculate_tax**](PaymentsApi.md#calculate_tax) | **POST** /billing/v1/tax/calculate | CalculateTax
[**create_checkout_session**](PaymentsApi.md#create_checkout_session) | **POST** /billing/v1/checkout | CreateCheckoutSession
[**create_setup_intent**](PaymentsApi.md#create_setup_intent) | **POST** /billing/v1/setup-intent | CreateSetupIntent
[**delete_payment_method**](PaymentsApi.md#delete_payment_method) | **DELETE** /billing/v1/payment-methods/{payment_method_id} | DeletePaymentMethod
[**get_billing_address**](PaymentsApi.md#get_billing_address) | **GET** /billing/v1/address | GetBillingAddress
[**get_billing_config**](PaymentsApi.md#get_billing_config) | **GET** /billing/v1/config | GetBillingConfig
[**get_payment_method_from_payment_intent**](PaymentsApi.md#get_payment_method_from_payment_intent) | **GET** /billing/v1/payment-methods/from-payment-intent | GetPaymentMethodFromPaymentIntent
[**get_payment_method_from_setup_intent**](PaymentsApi.md#get_payment_method_from_setup_intent) | **GET** /billing/v1/payment-methods/from-setup-intent | GetPaymentMethodFromSetupIntent
[**list_payment_methods**](PaymentsApi.md#list_payment_methods) | **GET** /billing/v1/payment-methods | ListPaymentMethods
[**upgrade_subscription**](PaymentsApi.md#upgrade_subscription) | **POST** /billing/v1/subscription/upgrade | UpgradeSubscription
[**upsert_billing_address**](PaymentsApi.md#upsert_billing_address) | **PUT** /billing/v1/address | UpsertBillingAddress
[**upsert_payment_method**](PaymentsApi.md#upsert_payment_method) | **PUT** /billing/v1/payment-methods | UpsertPaymentMethod



## calculate_generic_tax

> models::BillingV1TaxCalculationResponse calculate_generic_tax(billing_v1_calculate_generic_tax_request)
CalculateGenericTax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_calculate_generic_tax_request** | [**BillingV1CalculateGenericTaxRequest**](BillingV1CalculateGenericTaxRequest.md) |  | [required] |

### Return type

[**models::BillingV1TaxCalculationResponse**](billing.v1.TaxCalculationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calculate_tax

> models::BillingV1PlanTaxCalculationResponse calculate_tax(billing_v1_calculate_tax_request)
CalculateTax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_calculate_tax_request** | [**BillingV1CalculateTaxRequest**](BillingV1CalculateTaxRequest.md) |  | [required] |

### Return type

[**models::BillingV1PlanTaxCalculationResponse**](billing.v1.PlanTaxCalculationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_checkout_session

> models::BillingV1HostedCheckoutSessionResponse create_checkout_session(billing_v1_create_checkout_session_request)
CreateCheckoutSession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_create_checkout_session_request** | [**BillingV1CreateCheckoutSessionRequest**](BillingV1CreateCheckoutSessionRequest.md) |  | [required] |

### Return type

[**models::BillingV1HostedCheckoutSessionResponse**](billing.v1.HostedCheckoutSessionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_setup_intent

> models::BillingV1SetupIntentResponse create_setup_intent(billing_v1_create_setup_intent_request)
CreateSetupIntent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_create_setup_intent_request** | [**BillingV1CreateSetupIntentRequest**](BillingV1CreateSetupIntentRequest.md) |  | [required] |

### Return type

[**models::BillingV1SetupIntentResponse**](billing.v1.SetupIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_method

> serde_json::Value delete_payment_method(payment_method_id, org_id)
DeletePaymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_method_id** | **String** | The payment_method_id path parameter. | [required] |
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_address

> models::BillingV1BillingAddress get_billing_address(org_id)
GetBillingAddress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**models::BillingV1BillingAddress**](billing.v1.BillingAddress.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_config

> models::BillingV1BillingConfig get_billing_config()
GetBillingConfig

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BillingV1BillingConfig**](billing.v1.BillingConfig.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method_from_payment_intent

> models::BillingV1PaymentMethodIdResponse get_payment_method_from_payment_intent(payment_intent_id)
GetPaymentMethodFromPaymentIntent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_intent_id** | Option<**String**> | The payment_intent_id query parameter. |  |

### Return type

[**models::BillingV1PaymentMethodIdResponse**](billing.v1.PaymentMethodIdResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_method_from_setup_intent

> models::BillingV1PaymentMethodIdResponse get_payment_method_from_setup_intent(setup_intent_id)
GetPaymentMethodFromSetupIntent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setup_intent_id** | Option<**String**> | The setup_intent_id query parameter. |  |

### Return type

[**models::BillingV1PaymentMethodIdResponse**](billing.v1.PaymentMethodIdResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_methods

> models::BillingV1ListPaymentMethodsResponse list_payment_methods(org_id, refresh)
ListPaymentMethods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**refresh** | Option<**bool**> | The refresh query parameter. |  |

### Return type

[**models::BillingV1ListPaymentMethodsResponse**](billing.v1.ListPaymentMethodsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_subscription

> models::BillingV1UpgradeSubscriptionResponse upgrade_subscription(billing_v1_upgrade_subscription_request)
UpgradeSubscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_upgrade_subscription_request** | [**BillingV1UpgradeSubscriptionRequest**](BillingV1UpgradeSubscriptionRequest.md) |  | [required] |

### Return type

[**models::BillingV1UpgradeSubscriptionResponse**](billing.v1.UpgradeSubscriptionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_billing_address

> models::BillingV1BillingAddress upsert_billing_address(billing_v1_upsert_billing_address_request)
UpsertBillingAddress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_upsert_billing_address_request** | [**BillingV1UpsertBillingAddressRequest**](BillingV1UpsertBillingAddressRequest.md) |  | [required] |

### Return type

[**models::BillingV1BillingAddress**](billing.v1.BillingAddress.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_payment_method

> models::BillingV1PaymentMethodDetails upsert_payment_method(billing_v1_upsert_payment_method_request)
UpsertPaymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_v1_upsert_payment_method_request** | [**BillingV1UpsertPaymentMethodRequest**](BillingV1UpsertPaymentMethodRequest.md) |  | [required] |

### Return type

[**models::BillingV1PaymentMethodDetails**](billing.v1.PaymentMethodDetails.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

