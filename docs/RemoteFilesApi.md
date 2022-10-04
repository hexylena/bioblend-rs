# \RemoteFilesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_api_ftp_files_get**](RemoteFilesApi.md#index_api_ftp_files_get) | **GET** /api/ftp_files | Displays remote files available to the user. Please use /api/remote_files instead.
[**index_api_ftp_files_get_0**](RemoteFilesApi.md#index_api_ftp_files_get_0) | **GET** /api/ftp_files | Displays remote files available to the user. Please use /api/remote_files instead.
[**index_api_remote_files_get**](RemoteFilesApi.md#index_api_remote_files_get) | **GET** /api/remote_files | Displays remote files available to the user.
[**index_api_remote_files_get_0**](RemoteFilesApi.md#index_api_remote_files_get_0) | **GET** /api/remote_files | Displays remote files available to the user.
[**plugins_api_remote_files_plugins_get**](RemoteFilesApi.md#plugins_api_remote_files_plugins_get) | **GET** /api/remote_files/plugins | Display plugin information for each of the gxfiles:// URI targets available.
[**plugins_api_remote_files_plugins_get_0**](RemoteFilesApi.md#plugins_api_remote_files_plugins_get_0) | **GET** /api/remote_files/plugins | Display plugin information for each of the gxfiles:// URI targets available.



## index_api_ftp_files_get

> Vec<serde_json::Value> index_api_ftp_files_get(target, format, recursive, disable, run_as)
Displays remote files available to the user. Please use /api/remote_files instead.

Lists all remote files available to the user from different sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | Option<**String**> | The source to load datasets from. Possible values: ftpdir, userdir, importdir |  |[default to ftpdir]
**format** | Option<[**crate::models::RemoteFilesFormat**](.md)> | The requested format of returned data. Either `flat` to simply list all the files, `jstree` to get a tree representation of the files, or the default `uri` to list files and directories by their URI. |  |
**recursive** | Option<**bool**> | Wether to recursively lists all sub-directories. This will be `True` by default depending on the `target`. |  |
**disable** | Option<[**crate::models::RemoteFilesDisableMode**](.md)> | (This only applies when `format` is `jstree`) The value can be either `folders` or `files` and it will disable the corresponding nodes of the tree. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_ftp_files_get_0

> Vec<serde_json::Value> index_api_ftp_files_get_0(target, format, recursive, disable, run_as)
Displays remote files available to the user. Please use /api/remote_files instead.

Lists all remote files available to the user from different sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | Option<**String**> | The source to load datasets from. Possible values: ftpdir, userdir, importdir |  |[default to ftpdir]
**format** | Option<[**crate::models::RemoteFilesFormat**](.md)> | The requested format of returned data. Either `flat` to simply list all the files, `jstree` to get a tree representation of the files, or the default `uri` to list files and directories by their URI. |  |
**recursive** | Option<**bool**> | Wether to recursively lists all sub-directories. This will be `True` by default depending on the `target`. |  |
**disable** | Option<[**crate::models::RemoteFilesDisableMode**](.md)> | (This only applies when `format` is `jstree`) The value can be either `folders` or `files` and it will disable the corresponding nodes of the tree. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_remote_files_get

> Vec<serde_json::Value> index_api_remote_files_get(target, format, recursive, disable, run_as)
Displays remote files available to the user.

Lists all remote files available to the user from different sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | Option<**String**> | The source to load datasets from. Possible values: ftpdir, userdir, importdir |  |[default to ftpdir]
**format** | Option<[**crate::models::RemoteFilesFormat**](.md)> | The requested format of returned data. Either `flat` to simply list all the files, `jstree` to get a tree representation of the files, or the default `uri` to list files and directories by their URI. |  |
**recursive** | Option<**bool**> | Wether to recursively lists all sub-directories. This will be `True` by default depending on the `target`. |  |
**disable** | Option<[**crate::models::RemoteFilesDisableMode**](.md)> | (This only applies when `format` is `jstree`) The value can be either `folders` or `files` and it will disable the corresponding nodes of the tree. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_remote_files_get_0

> Vec<serde_json::Value> index_api_remote_files_get_0(target, format, recursive, disable, run_as)
Displays remote files available to the user.

Lists all remote files available to the user from different sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | Option<**String**> | The source to load datasets from. Possible values: ftpdir, userdir, importdir |  |[default to ftpdir]
**format** | Option<[**crate::models::RemoteFilesFormat**](.md)> | The requested format of returned data. Either `flat` to simply list all the files, `jstree` to get a tree representation of the files, or the default `uri` to list files and directories by their URI. |  |
**recursive** | Option<**bool**> | Wether to recursively lists all sub-directories. This will be `True` by default depending on the `target`. |  |
**disable** | Option<[**crate::models::RemoteFilesDisableMode**](.md)> | (This only applies when `format` is `jstree`) The value can be either `folders` or `files` and it will disable the corresponding nodes of the tree. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_api_remote_files_plugins_get

> Vec<crate::models::FilesSourcePlugin> plugins_api_remote_files_plugins_get(run_as)
Display plugin information for each of the gxfiles:// URI targets available.

Display plugin information for each of the gxfiles:// URI targets available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::FilesSourcePlugin>**](FilesSourcePlugin.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_api_remote_files_plugins_get_0

> Vec<crate::models::FilesSourcePlugin> plugins_api_remote_files_plugins_get_0(run_as)
Display plugin information for each of the gxfiles:// URI targets available.

Display plugin information for each of the gxfiles:// URI targets available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::FilesSourcePlugin>**](FilesSourcePlugin.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

