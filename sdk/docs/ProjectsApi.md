# \ProjectsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /organizations/{org_id}/projects/v1 | CreateProject
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /organizations/{org_id}/projects/v1/{project_id} | DeleteProject
[**get_project**](ProjectsApi.md#get_project) | **GET** /organizations/{org_id}/projects/v1/{project_id} | GetProject
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /organizations/{org_id}/projects/v1 | ListProjects
[**move_project**](ProjectsApi.md#move_project) | **POST** /organizations/{org_id}/projects/v1/{project_id}/move | MoveProject
[**remove_custom_domain**](ProjectsApi.md#remove_custom_domain) | **DELETE** /organizations/{org_id}/projects/v1/{project_id}/custom-domain | RemoveCustomDomain
[**set_custom_domain**](ProjectsApi.md#set_custom_domain) | **PUT** /organizations/{org_id}/projects/v1/{project_id}/custom-domain | SetCustomDomain
[**update_project_name**](ProjectsApi.md#update_project_name) | **PATCH** /organizations/{org_id}/projects/v1/{project_id}/name | UpdateProjectName
[**update_video_quality**](ProjectsApi.md#update_video_quality) | **PATCH** /organizations/{org_id}/projects/v1/{project_id}/video-quality | UpdateVideoQuality



## create_project

> models::ProjectV1Project create_project(org_id, create_project_request)
CreateProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**create_project_request** | [**CreateProjectRequest**](CreateProjectRequest.md) |  | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> models::ProjectV1Project delete_project(org_id, project_id)
DeleteProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::ProjectV1Project get_project(org_id, project_id)
GetProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> models::ProjectV1ListProjectsResponse list_projects(org_id)
ListProjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |

### Return type

[**models::ProjectV1ListProjectsResponse**](project.v1.ListProjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_project

> models::ProjectV1Project move_project(org_id, project_id, move_project_request)
MoveProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |
**move_project_request** | [**MoveProjectRequest**](MoveProjectRequest.md) |  | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_custom_domain

> models::ProjectV1Project remove_custom_domain(org_id, project_id)
RemoveCustomDomain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_custom_domain

> models::ProjectV1Project set_custom_domain(org_id, project_id, set_custom_domain_request)
SetCustomDomain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |
**set_custom_domain_request** | [**SetCustomDomainRequest**](SetCustomDomainRequest.md) |  | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project_name

> models::ProjectV1Project update_project_name(org_id, project_id, update_project_name_request)
UpdateProjectName

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |
**update_project_name_request** | [**UpdateProjectNameRequest**](UpdateProjectNameRequest.md) |  | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_video_quality

> models::ProjectV1Project update_video_quality(org_id, project_id, update_video_quality_request)
UpdateVideoQuality

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The org_id path parameter. | [required] |
**project_id** | **String** | The project_id path parameter. | [required] |
**update_video_quality_request** | [**UpdateVideoQualityRequest**](UpdateVideoQualityRequest.md) |  | [required] |

### Return type

[**models::ProjectV1Project**](project.v1.Project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

