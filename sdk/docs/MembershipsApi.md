# \MembershipsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation**](MembershipsApi.md#accept_invitation) | **POST** /auth/v1/invitations/{token}/accept | AcceptInvitation
[**cancel_invitation**](MembershipsApi.md#cancel_invitation) | **DELETE** /auth/v1/memberships/{user_org_id}/invite/{user_id} | CancelInvitation
[**check_membership**](MembershipsApi.md#check_membership) | **GET** /auth/v1/memberships/{org_id}/check | CheckMembership
[**decline_invitation**](MembershipsApi.md#decline_invitation) | **POST** /auth/v1/invitations/{token}/decline | DeclineInvitation
[**get_internal_membership_info**](MembershipsApi.md#get_internal_membership_info) | **GET** /auth/v1/memberships/{org_id}/info | GetInternalMembershipInfo
[**invite_member**](MembershipsApi.md#invite_member) | **POST** /auth/v1/memberships/{user_org_id}/invite | InviteMember
[**leave_organization**](MembershipsApi.md#leave_organization) | **POST** /auth/v1/memberships/{org_id}/leave | LeaveOrganization
[**list_membership_applications**](MembershipsApi.md#list_membership_applications) | **GET** /auth/v1/membership-applications | ListMembershipApplications
[**list_memberships**](MembershipsApi.md#list_memberships) | **GET** /auth/v1/memberships | ListMemberships
[**list_organization_members**](MembershipsApi.md#list_organization_members) | **GET** /auth/v1/memberships/{user_org_id}/members | ListOrganizationMembers
[**reactivate_member**](MembershipsApi.md#reactivate_member) | **PATCH** /auth/v1/memberships/{user_org_id}/members/{user_id}/reactivate | ReactivateMember
[**remove_member**](MembershipsApi.md#remove_member) | **DELETE** /auth/v1/memberships/{user_org_id}/members/{user_id} | RemoveMember
[**resend_invitation**](MembershipsApi.md#resend_invitation) | **POST** /auth/v1/memberships/{user_org_id}/invite/resend | ResendInvitation
[**respond_to_membership_application**](MembershipsApi.md#respond_to_membership_application) | **PATCH** /auth/v1/membership-applications/{org_id}/state | RespondToMembershipApplication
[**suspend_member**](MembershipsApi.md#suspend_member) | **PATCH** /auth/v1/memberships/{user_org_id}/members/{user_id}/suspend | SuspendMember
[**update_active_membership**](MembershipsApi.md#update_active_membership) | **PATCH** /auth/v1/memberships/active | UpdateActiveMembership
[**update_member_role**](MembershipsApi.md#update_member_role) | **PATCH** /auth/v1/memberships/{user_org_id}/members/{user_id}/role | UpdateMemberRole
[**update_membership_state**](MembershipsApi.md#update_membership_state) | **PATCH** /auth/v1/memberships/{user_org_id}/state | UpdateMembershipState
[**update_org_name**](MembershipsApi.md#update_org_name) | **PATCH** /auth/v1/memberships/{user_org_id}/name | UpdateOrgName
[**update_org_username**](MembershipsApi.md#update_org_username) | **PATCH** /auth/v1/memberships/{user_org_id}/username | UpdateOrgUsername



## accept_invitation

> models::AuthV1AcceptInvitationResponse accept_invitation(token)
AcceptInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The token path parameter. | [required] |

### Return type

[**models::AuthV1AcceptInvitationResponse**](auth.v1.AcceptInvitationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_invitation

> serde_json::Value cancel_invitation(user_org_id, user_id)
CancelInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**user_id** | **String** | The user_id path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_membership

> models::AuthV1CheckMembershipResponse check_membership(org_id, user_id)
CheckMembership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

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
**token** | **String** | The token path parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internal_membership_info

> models::AuthV1GetInternalMembershipInfoResponse get_internal_membership_info(org_id, user_id)
GetInternalMembershipInfo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**models::AuthV1GetInternalMembershipInfoResponse**](auth.v1.GetInternalMembershipInfoResponse.md)

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
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
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
**org_id** | **String** | The org_id path parameter. | [required] |
**user_id** | Option<**String**> | The user_id query parameter. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_membership_applications

> models::AuthV1ListMembershipApplicationsResponse list_membership_applications(user_user_id, limit, offset, state)
ListMembershipApplications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_user_id** | Option<**String**> | The user.user_id query parameter. |  |
**limit** | Option<**i32**> | The limit query parameter. |  |
**offset** | Option<**i32**> | The offset query parameter. |  |
**state** | Option<[**models::AuthV1MembershipApplicationState**](Models__AuthV1MembershipApplicationState.md)> | The state query parameter. |  |

### Return type

[**models::AuthV1ListMembershipApplicationsResponse**](auth.v1.ListMembershipApplicationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_memberships

> models::AuthV1ListMembershipsResponse list_memberships(user_user_id, limit, offset, state)
ListMemberships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_user_id** | Option<**String**> | The user.user_id query parameter. |  |
**limit** | Option<**i32**> | The limit query parameter. |  |
**offset** | Option<**i32**> | The offset query parameter. |  |
**state** | Option<[**models::AuthV1MembershipState**](Models__AuthV1MembershipState.md)> | The state query parameter. |  |

### Return type

[**models::AuthV1ListMembershipsResponse**](auth.v1.ListMembershipsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_members

> models::AuthV1ListOrgMembersResponse list_organization_members(user_org_id, limit, offset)
ListOrganizationMembers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**limit** | Option<**i32**> | The limit query parameter. |  |
**offset** | Option<**i32**> | The offset query parameter. |  |

### Return type

[**models::AuthV1ListOrgMembersResponse**](auth.v1.ListOrgMembersResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactivate_member

> models::AuthV1MembershipMutation reactivate_member(user_org_id, user_id)
ReactivateMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**user_id** | **String** | The user_id path parameter. | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_member

> serde_json::Value remove_member(user_org_id, user_id)
RemoveMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**user_id** | **String** | The user_id path parameter. | [required] |

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
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
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
**org_id** | **String** | The org_id path parameter. | [required] |
**respond_to_membership_application_request** | [**RespondToMembershipApplicationRequest**](RespondToMembershipApplicationRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suspend_member

> models::AuthV1MembershipMutation suspend_member(user_org_id, user_id)
SuspendMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**user_id** | **String** | The user_id path parameter. | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_active_membership

> models::AuthV1MembershipMutation update_active_membership(auth_v1_update_active_membership_request)
UpdateActiveMembership

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_update_active_membership_request** | [**AuthV1UpdateActiveMembershipRequest**](AuthV1UpdateActiveMembershipRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_member_role

> models::AuthV1MembershipMutation update_member_role(user_org_id, user_id, update_member_role_request)
UpdateMemberRole

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**user_id** | **String** | The user_id path parameter. | [required] |
**update_member_role_request** | [**UpdateMemberRoleRequest**](UpdateMemberRoleRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_membership_state

> models::AuthV1MembershipMutation update_membership_state(user_org_id, update_membership_state_request)
UpdateMembershipState

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**update_membership_state_request** | [**UpdateMembershipStateRequest**](UpdateMembershipStateRequest.md) |  | [required] |

### Return type

[**models::AuthV1MembershipMutation**](auth.v1.MembershipMutation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_name

> models::AuthV1UpdateOrgNameResponse update_org_name(user_org_id, update_org_name_request)
UpdateOrgName

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**update_org_name_request** | [**UpdateOrgNameRequest**](UpdateOrgNameRequest.md) |  | [required] |

### Return type

[**models::AuthV1UpdateOrgNameResponse**](auth.v1.UpdateOrgNameResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_username

> models::AuthV1UpdateOrgUsernameResponse update_org_username(user_org_id, update_org_username_request)
UpdateOrgUsername

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_org_id** | **String** | The user.org_id path parameter. | [required] |
**update_org_username_request** | [**UpdateOrgUsernameRequest**](UpdateOrgUsernameRequest.md) |  | [required] |

### Return type

[**models::AuthV1UpdateOrgUsernameResponse**](auth.v1.UpdateOrgUsernameResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

