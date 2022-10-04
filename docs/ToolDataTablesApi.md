# \ToolDataTablesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_api_tool_data_table_name_delete**](ToolDataTablesApi.md#delete_api_tool_data_table_name_delete) | **DELETE** /api/tool_data/{table_name} | Removes an item from a data table
[**delete_api_tool_data_table_name_delete_0**](ToolDataTablesApi.md#delete_api_tool_data_table_name_delete_0) | **DELETE** /api/tool_data/{table_name} | Removes an item from a data table
[**download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get**](ToolDataTablesApi.md#download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get) | **GET** /api/tool_data/{table_name}/fields/{field_name}/files/{file_name} | Get information about a particular field in a tool data table
[**download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get_0**](ToolDataTablesApi.md#download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get_0) | **GET** /api/tool_data/{table_name}/fields/{field_name}/files/{file_name} | Get information about a particular field in a tool data table
[**index_api_tool_data_get**](ToolDataTablesApi.md#index_api_tool_data_get) | **GET** /api/tool_data | Lists all available data tables
[**index_api_tool_data_get_0**](ToolDataTablesApi.md#index_api_tool_data_get_0) | **GET** /api/tool_data | Lists all available data tables
[**reload_api_tool_data_table_name_reload_get**](ToolDataTablesApi.md#reload_api_tool_data_table_name_reload_get) | **GET** /api/tool_data/{table_name}/reload | Reloads a tool data table
[**reload_api_tool_data_table_name_reload_get_0**](ToolDataTablesApi.md#reload_api_tool_data_table_name_reload_get_0) | **GET** /api/tool_data/{table_name}/reload | Reloads a tool data table
[**show_api_tool_data_table_name_get**](ToolDataTablesApi.md#show_api_tool_data_table_name_get) | **GET** /api/tool_data/{table_name} | Get details of a given data table
[**show_api_tool_data_table_name_get_0**](ToolDataTablesApi.md#show_api_tool_data_table_name_get_0) | **GET** /api/tool_data/{table_name} | Get details of a given data table
[**show_field_api_tool_data_table_name_fields_field_name_get**](ToolDataTablesApi.md#show_field_api_tool_data_table_name_fields_field_name_get) | **GET** /api/tool_data/{table_name}/fields/{field_name} | Get information about a particular field in a tool data table
[**show_field_api_tool_data_table_name_fields_field_name_get_0**](ToolDataTablesApi.md#show_field_api_tool_data_table_name_fields_field_name_get_0) | **GET** /api/tool_data/{table_name}/fields/{field_name} | Get information about a particular field in a tool data table



## delete_api_tool_data_table_name_delete

> crate::models::ToolDataDetails delete_api_tool_data_table_name_delete(table_name, tool_data_item, run_as)
Removes an item from a data table

Removes an item from a data table and reloads it to return its updated details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**tool_data_item** | [**ToolDataItem**](ToolDataItem.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_tool_data_table_name_delete_0

> crate::models::ToolDataDetails delete_api_tool_data_table_name_delete_0(table_name, tool_data_item, run_as)
Removes an item from a data table

Removes an item from a data table and reloads it to return its updated details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**tool_data_item** | [**ToolDataItem**](ToolDataItem.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get

> download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get(table_name, field_name, file_name, run_as)
Get information about a particular field in a tool data table

Download a file associated with the data table field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**field_name** | **String** | The name of the tool data table field | [required] |
**file_name** | **String** | The name of a file associated with this data table field | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get_0

> download_field_file_api_tool_data_table_name_fields_field_name_files_file_name_get_0(table_name, field_name, file_name, run_as)
Get information about a particular field in a tool data table

Download a file associated with the data table field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**field_name** | **String** | The name of the tool data table field | [required] |
**file_name** | **String** | The name of a file associated with this data table field | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_tool_data_get

> Vec<crate::models::ToolDataEntry> index_api_tool_data_get(run_as)
Lists all available data tables

Get the list of all available data tables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ToolDataEntry>**](ToolDataEntry.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_tool_data_get_0

> Vec<crate::models::ToolDataEntry> index_api_tool_data_get_0(run_as)
Lists all available data tables

Get the list of all available data tables.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ToolDataEntry>**](ToolDataEntry.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_api_tool_data_table_name_reload_get

> crate::models::ToolDataDetails reload_api_tool_data_table_name_reload_get(table_name, run_as)
Reloads a tool data table

Reloads a data table and return its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_api_tool_data_table_name_reload_get_0

> crate::models::ToolDataDetails reload_api_tool_data_table_name_reload_get_0(table_name, run_as)
Reloads a tool data table

Reloads a data table and return its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_tool_data_table_name_get

> crate::models::ToolDataDetails show_api_tool_data_table_name_get(table_name, run_as)
Get details of a given data table

Get details of a given tool data table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_tool_data_table_name_get_0

> crate::models::ToolDataDetails show_api_tool_data_table_name_get_0(table_name, run_as)
Get details of a given data table

Get details of a given tool data table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataDetails**](ToolDataDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_field_api_tool_data_table_name_fields_field_name_get

> crate::models::ToolDataField show_field_api_tool_data_table_name_fields_field_name_get(table_name, field_name, run_as)
Get information about a particular field in a tool data table

Reloads a data table and return its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**field_name** | **String** | The name of the tool data table field | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataField**](ToolDataField.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_field_api_tool_data_table_name_fields_field_name_get_0

> crate::models::ToolDataField show_field_api_tool_data_table_name_fields_field_name_get_0(table_name, field_name, run_as)
Get information about a particular field in a tool data table

Reloads a data table and return its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_name** | **String** | The name of the tool data table | [required] |
**field_name** | **String** | The name of the tool data table field | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ToolDataField**](ToolDataField.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

