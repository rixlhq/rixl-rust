# \AccessPoliciesApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_policy**](AccessPoliciesApi.md#attach_policy) | **POST** /auth/v1/memberships/{user_org_id}/policies/{policy_id}/attachments | AttachPolicy
[**create_policy**](AccessPoliciesApi.md#create_policy) | **POST** /auth/v1/memberships/{user_org_id}/policies | CreatePolicy
[**delete_policy**](AccessPoliciesApi.md#delete_policy) | **DELETE** /auth/v1/memberships/{user_org_id}/policies/{policy_id} | DeletePolicy
[**detach_policy**](AccessPoliciesApi.md#detach_policy) | **DELETE** /auth/v1/memberships/{user_org_id}/policies/attachments/{attachment_id} | DetachPolicy
[**get_policy**](AccessPoliciesApi.md#get_policy) | **GET** /auth/v1/memberships/{user_org_id}/policies/{policy_id} | GetPolicy
[**list_permission_registry**](AccessPoliciesApi.md#list_permission_registry) | **GET** /auth/v1/memberships/{user_org_id}/policies/permissions | ListPermissionRegistry
[**list_policies**](AccessPoliciesApi.md#list_policies) | **GET** /auth/v1/memberships/{org_id}/policies | ListPolicies
[**list_policy_attachments**](AccessPoliciesApi.md#list_policy_attachments) | **GET** /auth/v1/memberships/{user_org_id}/policies/{policy_id}/attachments | ListPolicyAttachments
[**list_user_policies**](AccessPoliciesApi.md#list_user_policies) | **GET** /auth/v1/memberships/{user_org_id}/members/{member_id}/policies | ListUserPolicies
[**update_policy**](AccessPoliciesApi.md#update_policy) | **PUT** /auth/v1/memberships/{user_org_id}/policies/{policy_id} | UpdatePolicy



## attach_policy

> models::AuthV1PolicyAttachment attach_policy(user_org_id, policy_id, attach_policy_request)
AttachPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**policy_id** | **String** | The policy_id path parameter. | [required] |
**attach_policy_request** | [**AttachPolicyRequest**](AttachPolicyRequest.md) |  | [required] |

### Return type

[**models::AuthV1PolicyAttachment**](auth.v1.PolicyAttachment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy

> models::AuthV1Policy create_policy(user_org_id, create_policy_request)
CreatePolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**create_policy_request** | [**CreatePolicyRequest**](CreatePolicyRequest.md) |  | [required] |

### Return type

[**models::AuthV1Policy**](auth.v1.Policy.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> serde_json::Value delete_policy(user_org_id, policy_id)
DeletePolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**policy_id** | **String** | The policy_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_policy

> serde_json::Value detach_policy(user_org_id, attachment_id)
DetachPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**attachment_id** | **String** | The attachment_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> models::AuthV1Policy get_policy(user_org_id, policy_id)
GetPolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**policy_id** | **String** | The policy_id path parameter. | [required] |

### Return type

[**models::AuthV1Policy**](auth.v1.Policy.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_permission_registry

> models::AuthV1PermissionRegistryResponse list_permission_registry(user_org_id, types, limit, offset)
ListPermissionRegistry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**types** | Option<[**Vec<String>**](String.md)> | The types query parameter. |  |
**limit** | Option<**i32**> | The limit query parameter. |  |
**offset** | Option<**i32**> | The offset query parameter. |  |

### Return type

[**models::AuthV1PermissionRegistryResponse**](auth.v1.PermissionRegistryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policies

> models::AuthV1ListPoliciesResponse list_policies(org_id, user_id)
ListPolicies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1ListPoliciesResponse**](auth.v1.ListPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policy_attachments

> models::AuthV1ListAttachmentsResponse list_policy_attachments(user_org_id, policy_id)
ListPolicyAttachments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**policy_id** | **String** | The policy_id path parameter. | [required] |

### Return type

[**models::AuthV1ListAttachmentsResponse**](auth.v1.ListAttachmentsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_policies

> models::AuthV1MemberPoliciesResponse list_user_policies(user_org_id, member_id)
ListUserPolicies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**member_id** | **String** | The member_id path parameter. | [required] |

### Return type

[**models::AuthV1MemberPoliciesResponse**](auth.v1.MemberPoliciesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> models::AuthV1Policy update_policy(user_org_id, policy_id, update_policy_request)
UpdatePolicy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**policy_id** | **String** | The policy_id path parameter. | [required] |
**update_policy_request** | [**UpdatePolicyRequest**](UpdatePolicyRequest.md) |  | [required] |

### Return type

[**models::AuthV1Policy**](auth.v1.Policy.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

