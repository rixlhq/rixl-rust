# \DashboardsApi

All URIs are relative to *https://api.rixl.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_query_chart**](DashboardsApi.md#batch_query_chart) | **POST** /analytics/v1/dashboard/chart-query/batch | BatchQueryChart
[**create_dashboard**](DashboardsApi.md#create_dashboard) | **POST** /analytics/v1/dashboards | CreateDashboard
[**create_widget**](DashboardsApi.md#create_widget) | **POST** /analytics/v1/dashboards/{dashboard_id}/widgets | CreateWidget
[**delete_dashboard**](DashboardsApi.md#delete_dashboard) | **DELETE** /analytics/v1/dashboards/{id} | DeleteDashboard
[**delete_widget**](DashboardsApi.md#delete_widget) | **DELETE** /analytics/v1/dashboards/widgets/{id} | DeleteWidget
[**export_dashboard**](DashboardsApi.md#export_dashboard) | **POST** /analytics/v1/dashboards/{dashboard_id}/export | ExportDashboard
[**get_dashboard**](DashboardsApi.md#get_dashboard) | **GET** /analytics/v1/dashboards/{id} | GetDashboard
[**get_dashboard_stats**](DashboardsApi.md#get_dashboard_stats) | **GET** /analytics/v1/dashboard | GetDashboardStats
[**get_filter_options**](DashboardsApi.md#get_filter_options) | **POST** /analytics/v1/dashboard/filter-options | GetFilterOptions
[**get_scope_tree**](DashboardsApi.md#get_scope_tree) | **POST** /analytics/v1/dashboard/scope-tree | GetScopeTree
[**list_dashboards**](DashboardsApi.md#list_dashboards) | **GET** /analytics/v1/dashboards | ListDashboards
[**list_datasets**](DashboardsApi.md#list_datasets) | **GET** /analytics/v1/dashboard/datasets | ListDatasets
[**query_chart**](DashboardsApi.md#query_chart) | **POST** /analytics/v1/dashboard/chart-query | QueryChart
[**set_default_dashboard**](DashboardsApi.md#set_default_dashboard) | **POST** /analytics/v1/dashboards/{id}/default | SetDefaultDashboard
[**update_dashboard**](DashboardsApi.md#update_dashboard) | **PATCH** /analytics/v1/dashboards/{id} | UpdateDashboard
[**update_dashboard_layout**](DashboardsApi.md#update_dashboard_layout) | **POST** /analytics/v1/dashboards/{dashboard_id}/layout | UpdateDashboardLayout
[**update_widget**](DashboardsApi.md#update_widget) | **PATCH** /analytics/v1/dashboards/widgets/{id} | UpdateWidget



## batch_query_chart

> models::AnalyticsV1BatchChartQueryResponse batch_query_chart(analytics_v1_batch_chart_query_request)
BatchQueryChart

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_batch_chart_query_request** | [**AnalyticsV1BatchChartQueryRequest**](AnalyticsV1BatchChartQueryRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1BatchChartQueryResponse**](analytics.v1.BatchChartQueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dashboard

> models::AnalyticsV1Dashboard create_dashboard(analytics_v1_create_dashboard_request)
CreateDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_create_dashboard_request** | [**AnalyticsV1CreateDashboardRequest**](AnalyticsV1CreateDashboardRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1Dashboard**](analytics.v1.Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_widget

> models::AnalyticsV1Widget create_widget(dashboard_id, expected_revision, analytics_v1_widget_input)
CreateWidget

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The dashboard_id path parameter. | [required] |
**expected_revision** | **i32** | The expected_revision query parameter. | [required] |
**analytics_v1_widget_input** | Option<[**AnalyticsV1WidgetInput**](AnalyticsV1WidgetInput.md)> |  |  |

### Return type

[**models::AnalyticsV1Widget**](analytics.v1.Widget.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard

> serde_json::Value delete_dashboard(id, expected_revision)
DeleteDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**expected_revision** | **i32** | The expected_revision query parameter. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_widget

> models::AnalyticsV1DeleteWidgetResponse delete_widget(id, expected_revision)
DeleteWidget

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**expected_revision** | **i32** | The expected_revision query parameter. | [required] |

### Return type

[**models::AnalyticsV1DeleteWidgetResponse**](analytics.v1.DeleteWidgetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_dashboard

> models::AnalyticsV1ExportDashboardResponse export_dashboard(dashboard_id, export_dashboard_request)
ExportDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The dashboard_id path parameter. | [required] |
**export_dashboard_request** | [**ExportDashboardRequest**](ExportDashboardRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1ExportDashboardResponse**](analytics.v1.ExportDashboardResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard

> models::AnalyticsV1Dashboard get_dashboard(id)
GetDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |

### Return type

[**models::AnalyticsV1Dashboard**](analytics.v1.Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_stats

> models::AnalyticsV1DashboardStatsResponse get_dashboard_stats(time_start, time_end, interval, filter_countries, filter_cities, filter_devices, filter_os, filter_languages, filter_regions, filter_browsers, filter_os_versions)
GetDashboardStats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_start** | **String** | The time_start query parameter. | [required] |
**time_end** | **String** | The time_end query parameter. | [required] |
**interval** | Option<**String**> | The interval query parameter. |  |
**filter_countries** | Option<[**Vec<String>**](String.md)> | The filter.countries query parameter. |  |
**filter_cities** | Option<[**Vec<String>**](String.md)> | The filter.cities query parameter. |  |
**filter_devices** | Option<[**Vec<String>**](String.md)> | The filter.devices query parameter. |  |
**filter_os** | Option<[**Vec<String>**](String.md)> | The filter.os query parameter. |  |
**filter_languages** | Option<[**Vec<String>**](String.md)> | The filter.languages query parameter. |  |
**filter_regions** | Option<[**Vec<String>**](String.md)> | The filter.regions query parameter. |  |
**filter_browsers** | Option<[**Vec<String>**](String.md)> | The filter.browsers query parameter. |  |
**filter_os_versions** | Option<[**Vec<String>**](String.md)> | The filter.os_versions query parameter. |  |

### Return type

[**models::AnalyticsV1DashboardStatsResponse**](analytics.v1.DashboardStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_filter_options

> models::AnalyticsV1GetFilterOptionsResponse get_filter_options(analytics_v1_get_filter_options_request)
GetFilterOptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_get_filter_options_request** | [**AnalyticsV1GetFilterOptionsRequest**](AnalyticsV1GetFilterOptionsRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1GetFilterOptionsResponse**](analytics.v1.GetFilterOptionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scope_tree

> models::AnalyticsV1GetScopeTreeResponse get_scope_tree(analytics_v1_get_scope_tree_request)
GetScopeTree

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_get_scope_tree_request** | [**AnalyticsV1GetScopeTreeRequest**](AnalyticsV1GetScopeTreeRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1GetScopeTreeResponse**](analytics.v1.GetScopeTreeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dashboards

> models::AnalyticsV1ListDashboardsResponse list_dashboards(page_size, page)
ListDashboards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The page_size query parameter. |  |
**page** | Option<**i32**> | The page query parameter. |  |

### Return type

[**models::AnalyticsV1ListDashboardsResponse**](analytics.v1.ListDashboardsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_datasets

> models::AnalyticsV1ListDatasetsResponse list_datasets()
ListDatasets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AnalyticsV1ListDatasetsResponse**](analytics.v1.ListDatasetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_chart

> models::AnalyticsV1ChartQueryResponse query_chart(analytics_v1_chart_query_request)
QueryChart

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_v1_chart_query_request** | [**AnalyticsV1ChartQueryRequest**](AnalyticsV1ChartQueryRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1ChartQueryResponse**](analytics.v1.ChartQueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_dashboard

> models::AnalyticsV1Dashboard set_default_dashboard(id)
SetDefaultDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |

### Return type

[**models::AnalyticsV1Dashboard**](analytics.v1.Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard

> models::AnalyticsV1Dashboard update_dashboard(id, update_dashboard_request)
UpdateDashboard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**update_dashboard_request** | [**UpdateDashboardRequest**](UpdateDashboardRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1Dashboard**](analytics.v1.Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_layout

> models::AnalyticsV1Dashboard update_dashboard_layout(dashboard_id, update_dashboard_layout_request)
UpdateDashboardLayout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The dashboard_id path parameter. | [required] |
**update_dashboard_layout_request** | [**UpdateDashboardLayoutRequest**](UpdateDashboardLayoutRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1Dashboard**](analytics.v1.Dashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_widget

> models::AnalyticsV1Widget update_widget(id, update_widget_request)
UpdateWidget

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id path parameter. | [required] |
**update_widget_request** | [**UpdateWidgetRequest**](UpdateWidgetRequest.md) |  | [required] |

### Return type

[**models::AnalyticsV1Widget**](analytics.v1.Widget.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

