# \MembershipsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation**](MembershipsApi.md#accept_invitation) | **POST** /auth/v1/invitations/{token}/accept | AcceptInvitation
[**cancel_invitation**](MembershipsApi.md#cancel_invitation) | **DELETE** /auth/v1/organizations/{user_org_id}/invite/{user_id} | CancelInvitation
[**check_current_membership**](MembershipsApi.md#check_current_membership) | **GET** /auth/v1/organizations/{org_id}/members/current/check | CheckCurrentMembership
[**decline_invitation**](MembershipsApi.md#decline_invitation) | **POST** /auth/v1/invitations/{token}/decline | DeclineInvitation
[**invite_member**](MembershipsApi.md#invite_member) | **POST** /auth/v1/organizations/{user_org_id}/invite | InviteMember
[**leave_organization**](MembershipsApi.md#leave_organization) | **DELETE** /auth/v1/organizations/{org_id}/members/current | LeaveOrganization
[**list_membership_applications**](MembershipsApi.md#list_membership_applications) | **GET** /auth/v1/membership-applications | ListMembershipApplications
[**list_memberships**](MembershipsApi.md#list_memberships) | **GET** /auth/v1/users/current/memberships | ListMemberships
[**list_organization_members**](MembershipsApi.md#list_organization_members) | **GET** /auth/v1/organizations/{user_org_id}/members | ListOrganizationMembers
[**remove_member**](MembershipsApi.md#remove_member) | **DELETE** /auth/v1/organizations/{org_id}/members/{user_id} | RemoveMember
[**resend_invitation**](MembershipsApi.md#resend_invitation) | **POST** /auth/v1/organizations/{user_org_id}/invite/resend | ResendInvitation
[**respond_to_membership_application**](MembershipsApi.md#respond_to_membership_application) | **PATCH** /auth/v1/membership-applications/{org_id}/state | RespondToMembershipApplication
[**update_member**](MembershipsApi.md#update_member) | **PATCH** /auth/v1/organizations/{org_id}/members/{user_id} | UpdateMember
[**update_membership_state**](MembershipsApi.md#update_membership_state) | **PATCH** /auth/v1/organizations/{user_org_id}/membership-applications/{user_id} | UpdateMembershipState



## accept_invitation

> models::AuthV1AcceptInvitationResponse accept_invitation(token)
AcceptInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**models::AuthV1AcceptInvitationResponse**](auth.v1.AcceptInvitationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_invitation

> serde_json::Value cancel_invitation(user_org_id, user_id, user_actor_id)
CancelInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**user_actor_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## decline_invitation

> serde_json::Value decline_invitation(token)
DeclineInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_member

> models::AuthV1MembershipApplication invite_member(user_org_id, invite_member_request)
InviteMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**invite_member_request** | [**InviteMemberRequest**](InviteMemberRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipApplication**](auth.v1.MembershipApplication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
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


## list_membership_applications

> models::AuthV1ListMembershipApplicationsResponse list_membership_applications(limit, offset, state)
ListMembershipApplications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**state** | Option<[**models::AuthV1MembershipApplicationState**](Models__AuthV1MembershipApplicationState.md)> |  |  |

### Return type

[**models::AuthV1ListMembershipApplicationsResponse**](auth.v1.ListMembershipApplicationsResponse.md)

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

> models::AuthV1ListOrgMembersResponse list_organization_members(user_org_id, user_user_id, limit, offset)
ListOrganizationMembers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**user_user_id** | Option<**String**> |  |  |
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


## resend_invitation

> serde_json::Value resend_invitation(user_org_id, resend_invitation_request)
ResendInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**resend_invitation_request** | [**ResendInvitationRequest**](ResendInvitationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## respond_to_membership_application

> models::AuthV1MembershipMutation respond_to_membership_application(org_id, respond_to_membership_application_request)
RespondToMembershipApplication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**respond_to_membership_application_request** | [**RespondToMembershipApplicationRequest**](RespondToMembershipApplicationRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
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


## update_membership_state

> models::AuthV1MembershipMutation update_membership_state(user_org_id, user_id, update_membership_state_request)
UpdateMembershipState

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**update_membership_state_request** | [**UpdateMembershipStateRequest**](UpdateMembershipStateRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

