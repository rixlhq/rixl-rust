# \InvitationsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation**](InvitationsApi.md#accept_invitation) | **POST** /auth/v1/invitations/accept | AcceptInvitation
[**create_invitation**](InvitationsApi.md#create_invitation) | **POST** /auth/v1/organizations/{org_id}/invitations | CreateInvitation
[**decline_invitation**](InvitationsApi.md#decline_invitation) | **POST** /auth/v1/invitations/decline | DeclineInvitation
[**list_my_invitations**](InvitationsApi.md#list_my_invitations) | **GET** /auth/v1/users/current/invitations | ListMyInvitations
[**list_organization_invitations**](InvitationsApi.md#list_organization_invitations) | **GET** /auth/v1/organizations/{org_id}/invitations | ListOrganizationInvitations
[**lookup_invitation**](InvitationsApi.md#lookup_invitation) | **POST** /auth/v1/invitations/lookup | LookupInvitation
[**resend_invitation**](InvitationsApi.md#resend_invitation) | **POST** /auth/v1/organizations/{org_id}/invitations/{invitation_id}/resend | ResendInvitation
[**revoke_invitation**](InvitationsApi.md#revoke_invitation) | **DELETE** /auth/v1/organizations/{org_id}/invitations/{invitation_id} | RevokeInvitation



## accept_invitation

> models::AuthV1Membership accept_invitation(auth_v1_accept_invitation_request)
AcceptInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_accept_invitation_request** | [**AuthV1AcceptInvitationRequest**](AuthV1AcceptInvitationRequest.md) |  | [required] |

### Return type

[**models::AuthV1Membership**](auth.v1.Membership.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invitation

> models::AuthV1Invitation create_invitation(org_id, create_invitation_request)
CreateInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**create_invitation_request** | [**CreateInvitationRequest**](CreateInvitationRequest.md) |  | [required] |

### Return type

[**models::AuthV1Invitation**](auth.v1.Invitation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decline_invitation

> serde_json::Value decline_invitation(auth_v1_decline_invitation_request)
DeclineInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_decline_invitation_request** | [**AuthV1DeclineInvitationRequest**](AuthV1DeclineInvitationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_my_invitations

> models::AuthV1ListMyInvitationsResponse list_my_invitations(page_size, page_token)
ListMyInvitations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |

### Return type

[**models::AuthV1ListMyInvitationsResponse**](auth.v1.ListMyInvitationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_invitations

> models::AuthV1ListOrganizationInvitationsResponse list_organization_invitations(org_id, state, page_size, page_token)
ListOrganizationInvitations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**state** | Option<**String**> | Defaults to pending. |  |
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |

### Return type

[**models::AuthV1ListOrganizationInvitationsResponse**](auth.v1.ListOrganizationInvitationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lookup_invitation

> models::AuthV1LookupInvitationResponse lookup_invitation(auth_v1_lookup_invitation_request)
LookupInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_v1_lookup_invitation_request** | [**AuthV1LookupInvitationRequest**](AuthV1LookupInvitationRequest.md) |  | [required] |

### Return type

[**models::AuthV1LookupInvitationResponse**](auth.v1.LookupInvitationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_invitation

> models::AuthV1Invitation resend_invitation(org_id, invitation_id)
ResendInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**invitation_id** | **String** |  | [required] |

### Return type

[**models::AuthV1Invitation**](auth.v1.Invitation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_invitation

> serde_json::Value revoke_invitation(org_id, invitation_id)
RevokeInvitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**invitation_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

