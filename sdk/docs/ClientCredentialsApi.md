# \ClientCredentialsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client_credential**](ClientCredentialsApi.md#create_client_credential) | **POST** /platform/clientauth/v1/credentials | CreateClientCredential
[**list_client_credentials**](ClientCredentialsApi.md#list_client_credentials) | **GET** /platform/clientauth/v1/credentials | ListClientCredentials
[**mint_client_token**](ClientCredentialsApi.md#mint_client_token) | **POST** /platform/clientauth/v1/token | MintClientToken
[**revoke_client_credential**](ClientCredentialsApi.md#revoke_client_credential) | **POST** /platform/clientauth/v1/credentials/{credential_id}/revoke | RevokeClientCredential



## create_client_credential

> models::ClientauthV1CreateClientCredentialResponse create_client_credential(clientauth_v1_create_client_credential_request)
CreateClientCredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clientauth_v1_create_client_credential_request** | [**ClientauthV1CreateClientCredentialRequest**](ClientauthV1CreateClientCredentialRequest.md) |  | [required] |

### Return type

[**models::ClientauthV1CreateClientCredentialResponse**](clientauth.v1.CreateClientCredentialResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_client_credentials

> models::ClientauthV1ListClientCredentialsResponse list_client_credentials(org_id, pagination_limit, pagination_offset)
ListClientCredentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | Option<**String**> | The org_id query parameter. |  |
**pagination_limit** | Option<**i32**> | Maximum number of items to return. |  |[default to 25]
**pagination_offset** | Option<**i32**> | Number of items to skip before collecting the result set. |  |[default to 0]

### Return type

[**models::ClientauthV1ListClientCredentialsResponse**](clientauth.v1.ListClientCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_client_token

> models::ClientauthV1MintClientTokenResponse mint_client_token(clientauth_v1_mint_client_token_request)
MintClientToken

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clientauth_v1_mint_client_token_request** | [**ClientauthV1MintClientTokenRequest**](ClientauthV1MintClientTokenRequest.md) |  | [required] |

### Return type

[**models::ClientauthV1MintClientTokenResponse**](clientauth.v1.MintClientTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_client_credential

> serde_json::Value revoke_client_credential(credential_id, org_id)
RevokeClientCredential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The credential_id path parameter. | [required] |
**org_id** | Option<**String**> | The org_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

