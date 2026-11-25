# \CustomDomainsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_domain_verification**](CustomDomainsApi.md#check_domain_verification) | **POST** /auth/v1/organizations/{org_id}/domain/verify | CheckDomainVerification
[**create_domain_verification**](CustomDomainsApi.md#create_domain_verification) | **POST** /auth/v1/organizations/{org_id}/domain | CreateDomainVerification
[**get_domain_status**](CustomDomainsApi.md#get_domain_status) | **GET** /auth/v1/organizations/{org_id}/domain | GetDomainStatus
[**get_domain_suggest_join**](CustomDomainsApi.md#get_domain_suggest_join) | **GET** /auth/v1/organizations/{org_id}/domain/suggest-join | GetDomainSuggestJoin
[**join_suggested_organization**](CustomDomainsApi.md#join_suggested_organization) | **POST** /auth/v1/users/current/domain-join-suggestions/join | JoinSuggestedOrganization
[**list_domain_join_suggestions**](CustomDomainsApi.md#list_domain_join_suggestions) | **GET** /auth/v1/users/current/domain-join-suggestions | ListDomainJoinSuggestions
[**remove_domain**](CustomDomainsApi.md#remove_domain) | **DELETE** /auth/v1/organizations/{org_id}/domain | RemoveDomain
[**set_domain_suggest_join**](CustomDomainsApi.md#set_domain_suggest_join) | **PUT** /auth/v1/organizations/{org_id}/domain/suggest-join | SetDomainSuggestJoin



## check_domain_verification

> models::AuthV1DomainResponse check_domain_verification(org_id, user_id)
CheckDomainVerification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_domain_verification

> models::AuthV1DomainResponse create_domain_verification(org_id, create_domain_request)
CreateDomainVerification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**create_domain_request** | [**CreateDomainRequest**](CreateDomainRequest.md) |  | [required] |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_status

> models::AuthV1DomainResponse get_domain_status(org_id, user_id)
GetDomainStatus

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_suggest_join

> models::AuthV1SuggestJoinSetting get_domain_suggest_join(org_id, user_id)
GetDomainSuggestJoin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::AuthV1SuggestJoinSetting**](auth.v1.SuggestJoinSetting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_suggested_organization

> serde_json::Value join_suggested_organization(auth_v1_join_suggested_organization_request)
JoinSuggestedOrganization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_join_suggested_organization_request** | [**AuthV1JoinSuggestedOrganizationRequest**](AuthV1JoinSuggestedOrganizationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_domain_join_suggestions

> models::AuthV1ListDomainJoinSuggestionsResponse list_domain_join_suggestions(page_size, page_token)
ListDomainJoinSuggestions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |

### Return type

[**models::AuthV1ListDomainJoinSuggestionsResponse**](auth.v1.ListDomainJoinSuggestionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_domain

> serde_json::Value remove_domain(org_id, user_id)
RemoveDomain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_domain_suggest_join

> models::AuthV1SuggestJoinSetting set_domain_suggest_join(org_id, set_suggest_join_request)
SetDomainSuggestJoin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**set_suggest_join_request** | [**SetSuggestJoinRequest**](SetSuggestJoinRequest.md) |  | [required] |

### Return type

[**models::AuthV1SuggestJoinSetting**](auth.v1.SuggestJoinSetting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

