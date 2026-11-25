# \MembershipsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_current_membership**](MembershipsApi.md#check_current_membership) | **GET** /auth/v1/organizations/{org_id}/members/current/check | CheckCurrentMembership
[**leave_organization**](MembershipsApi.md#leave_organization) | **DELETE** /auth/v1/organizations/{org_id}/members/current | LeaveOrganization
[**list_memberships**](MembershipsApi.md#list_memberships) | **GET** /auth/v1/users/current/memberships | ListMemberships
[**list_organization_members**](MembershipsApi.md#list_organization_members) | **GET** /auth/v1/organizations/{org_id}/members | ListOrganizationMembers
[**remove_member**](MembershipsApi.md#remove_member) | **DELETE** /auth/v1/organizations/{org_id}/members/{user_id} | RemoveMember
[**update_member**](MembershipsApi.md#update_member) | **PATCH** /auth/v1/organizations/{org_id}/members/{user_id} | UpdateMember



## check_current_membership

> models::AuthV1CheckMembershipResponse check_current_membership(org_id)
CheckCurrentMembership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**models::AuthV1CheckMembershipResponse**](auth.v1.CheckMembershipResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_organization

> serde_json::Value leave_organization(org_id, user_id)
LeaveOrganization

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


## list_memberships

> models::AuthV1ListMembershipsResponse list_memberships(limit, offset, state)
ListMemberships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**state** | Option<[**models::AuthV1MembershipState**](Models__AuthV1MembershipState.md)> |  |  |

### Return type

[**models::AuthV1ListMembershipsResponse**](auth.v1.ListMembershipsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_members

> models::AuthV1ListOrgMembersResponse list_organization_members(org_id, limit, offset)
ListOrganizationMembers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::AuthV1ListOrgMembersResponse**](auth.v1.ListOrgMembersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_member

> serde_json::Value remove_member(org_id, user_id)
RemoveMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_member

> models::AuthV1MembershipMutation update_member(org_id, user_id, update_member_request)
UpdateMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**update_member_request** | [**UpdateMemberRequest**](UpdateMemberRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

