# \CustomDomainsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_domain_verification**](CustomDomainsApi.md#check_domain_verification) | **POST** /auth/v1/memberships/{org_id}/domain/verify | CheckDomainVerification
[**create_domain_verification**](CustomDomainsApi.md#create_domain_verification) | **POST** /auth/v1/memberships/{user_org_id}/domain | CreateDomainVerification
[**get_domain_auto_join**](CustomDomainsApi.md#get_domain_auto_join) | **GET** /auth/v1/memberships/{org_id}/domain/auto-join | GetDomainAutoJoin
[**get_domain_status**](CustomDomainsApi.md#get_domain_status) | **GET** /auth/v1/memberships/{org_id}/domain | GetDomainStatus
[**remove_domain**](CustomDomainsApi.md#remove_domain) | **DELETE** /auth/v1/memberships/{org_id}/domain | RemoveDomain
[**set_domain_auto_join**](CustomDomainsApi.md#set_domain_auto_join) | **PUT** /auth/v1/memberships/{user_org_id}/domain/auto-join | SetDomainAutoJoin



## check_domain_verification

> models::AuthV1DomainResponse check_domain_verification(org_id, user_id)
CheckDomainVerification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_domain_verification

> models::AuthV1DomainResponse create_domain_verification(user_org_id, create_domain_request)
CreateDomainVerification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**create_domain_request** | [**CreateDomainRequest**](CreateDomainRequest.md) |  | [required] |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_auto_join

> models::AuthV1AutoJoinSetting get_domain_auto_join(org_id, user_id)
GetDomainAutoJoin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1AutoJoinSetting**](auth.v1.AutoJoinSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_status

> models::AuthV1DomainResponse get_domain_status(org_id, user_id)
GetDomainStatus

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1DomainResponse**](auth.v1.DomainResponse.md)

### Authorization

No authorization required

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
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_domain_auto_join

> models::AuthV1AutoJoinSetting set_domain_auto_join(user_org_id, set_auto_join_request)
SetDomainAutoJoin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**set_auto_join_request** | [**SetAutoJoinRequest**](SetAutoJoinRequest.md) |  | [required] |

### Return type

[**models::AuthV1AutoJoinSetting**](auth.v1.AutoJoinSetting.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

