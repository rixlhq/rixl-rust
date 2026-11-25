# \OrganizationsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_avatar_upload**](OrganizationsApi.md#create_organization_avatar_upload) | **POST** /auth/v1/organizations/{org_id}/avatar/upload | CreateOrganizationAvatarUpload
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /auth/v1/organizations/{org_id} | GetOrganization
[**list_organization_audit_events**](OrganizationsApi.md#list_organization_audit_events) | **GET** /auth/v1/organizations/{org_id}/audit-events | ListOrganizationAuditEvents
[**list_organizations**](OrganizationsApi.md#list_organizations) | **GET** /auth/v1/organizations | ListOrganizations
[**update_organization**](OrganizationsApi.md#update_organization) | **PATCH** /auth/v1/organizations/{org_id} | UpdateOrganization



## create_organization_avatar_upload

> models::AuthV1CreateAvatarUploadResponse create_organization_avatar_upload(org_id)
CreateOrganizationAvatarUpload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**models::AuthV1CreateAvatarUploadResponse**](auth.v1.CreateAvatarUploadResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::AuthV1Organization get_organization(org_id)
GetOrganization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**models::AuthV1Organization**](auth.v1.Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_audit_events

> models::AuthV1ListOrganizationAuditEventsResponse list_organization_audit_events(org_id, page_size, page_token)
ListOrganizationAuditEvents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |

### Return type

[**models::AuthV1ListOrganizationAuditEventsResponse**](auth.v1.ListOrganizationAuditEventsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organizations

> models::AuthV1ListOrganizationsResponse list_organizations(page_size, page_token, show_deleted)
ListOrganizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |
**show_deleted** | Option<**bool**> | Includes deleted organizations visible to their owners once lifecycle adds deletion. |  |

### Return type

[**models::AuthV1ListOrganizationsResponse**](auth.v1.ListOrganizationsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::AuthV1Organization update_organization(org_id, update_organization_request)
UpdateOrganization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**update_organization_request** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md) |  | [required] |

### Return type

[**models::AuthV1Organization**](auth.v1.Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

