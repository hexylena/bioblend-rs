# \ConfigurationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_id_api_configuration_decode_encoded_id_get**](ConfigurationApi.md#decode_id_api_configuration_decode_encoded_id_get) | **GET** /api/configuration/decode/{encoded_id} | Decode a given id
[**decode_id_api_configuration_decode_encoded_id_get_0**](ConfigurationApi.md#decode_id_api_configuration_decode_encoded_id_get_0) | **GET** /api/configuration/decode/{encoded_id} | Decode a given id
[**dynamic_tool_confs_api_configuration_dynamic_tool_confs_get**](ConfigurationApi.md#dynamic_tool_confs_api_configuration_dynamic_tool_confs_get) | **GET** /api/configuration/dynamic_tool_confs | Return dynamic tool configuration files
[**dynamic_tool_confs_api_configuration_dynamic_tool_confs_get_0**](ConfigurationApi.md#dynamic_tool_confs_api_configuration_dynamic_tool_confs_get_0) | **GET** /api/configuration/dynamic_tool_confs | Return dynamic tool configuration files
[**index_api_configuration_get**](ConfigurationApi.md#index_api_configuration_get) | **GET** /api/configuration | Return an object containing exposable configuration settings
[**index_api_configuration_get_0**](ConfigurationApi.md#index_api_configuration_get_0) | **GET** /api/configuration | Return an object containing exposable configuration settings
[**reload_toolbox_api_configuration_toolbox_put**](ConfigurationApi.md#reload_toolbox_api_configuration_toolbox_put) | **PUT** /api/configuration/toolbox | Reload the Galaxy toolbox (but not individual tools)
[**reload_toolbox_api_configuration_toolbox_put_0**](ConfigurationApi.md#reload_toolbox_api_configuration_toolbox_put_0) | **PUT** /api/configuration/toolbox | Reload the Galaxy toolbox (but not individual tools)
[**tool_lineages_api_configuration_tool_lineages_get**](ConfigurationApi.md#tool_lineages_api_configuration_tool_lineages_get) | **GET** /api/configuration/tool_lineages | Return tool lineages for tools that have them
[**tool_lineages_api_configuration_tool_lineages_get_0**](ConfigurationApi.md#tool_lineages_api_configuration_tool_lineages_get_0) | **GET** /api/configuration/tool_lineages | Return tool lineages for tools that have them
[**version_api_version_get**](ConfigurationApi.md#version_api_version_get) | **GET** /api/version | Return Galaxy version information: major/minor version, optional extra info
[**version_api_version_get_0**](ConfigurationApi.md#version_api_version_get_0) | **GET** /api/version | Return Galaxy version information: major/minor version, optional extra info
[**whoami_api_whoami_get**](ConfigurationApi.md#whoami_api_whoami_get) | **GET** /api/whoami | Return information about the current authenticated user
[**whoami_api_whoami_get_0**](ConfigurationApi.md#whoami_api_whoami_get_0) | **GET** /api/whoami | Return information about the current authenticated user



## decode_id_api_configuration_decode_encoded_id_get

> ::std::collections::HashMap<String, i32> decode_id_api_configuration_decode_encoded_id_get(encoded_id, run_as)
Decode a given id

Decode a given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**encoded_id** | **String** | Encoded id to be decoded | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**::std::collections::HashMap<String, i32>**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_id_api_configuration_decode_encoded_id_get_0

> ::std::collections::HashMap<String, i32> decode_id_api_configuration_decode_encoded_id_get_0(encoded_id, run_as)
Decode a given id

Decode a given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**encoded_id** | **String** | Encoded id to be decoded | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

**::std::collections::HashMap<String, i32>**

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_tool_confs_api_configuration_dynamic_tool_confs_get

> Vec<::std::collections::HashMap<String, String>> dynamic_tool_confs_api_configuration_dynamic_tool_confs_get(run_as)
Return dynamic tool configuration files

Return dynamic tool configuration files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<::std::collections::HashMap<String, String>>**](map.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_tool_confs_api_configuration_dynamic_tool_confs_get_0

> Vec<::std::collections::HashMap<String, String>> dynamic_tool_confs_api_configuration_dynamic_tool_confs_get_0(run_as)
Return dynamic tool configuration files

Return dynamic tool configuration files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<::std::collections::HashMap<String, String>>**](map.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_configuration_get

> serde_json::Value index_api_configuration_get(view, keys, run_as)
Return an object containing exposable configuration settings

Return an object containing exposable configuration settings.  A more complete list is returned if the user is an admin. Pass in `view` and a comma-seperated list of keys to control which configuration settings are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_configuration_get_0

> serde_json::Value index_api_configuration_get_0(view, keys, run_as)
Return an object containing exposable configuration settings

Return an object containing exposable configuration settings.  A more complete list is returned if the user is an admin. Pass in `view` and a comma-seperated list of keys to control which configuration settings are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_toolbox_api_configuration_toolbox_put

> serde_json::Value reload_toolbox_api_configuration_toolbox_put(run_as)
Reload the Galaxy toolbox (but not individual tools)

Reload the Galaxy toolbox (but not individual tools).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_toolbox_api_configuration_toolbox_put_0

> serde_json::Value reload_toolbox_api_configuration_toolbox_put_0(run_as)
Reload the Galaxy toolbox (but not individual tools)

Reload the Galaxy toolbox (but not individual tools).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_lineages_api_configuration_tool_lineages_get

> Vec<::std::collections::HashMap<String, serde_json::Value>> tool_lineages_api_configuration_tool_lineages_get(run_as)
Return tool lineages for tools that have them

Return tool lineages for tools that have them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tool_lineages_api_configuration_tool_lineages_get_0

> Vec<::std::collections::HashMap<String, serde_json::Value>> tool_lineages_api_configuration_tool_lineages_get_0(run_as)
Return tool lineages for tools that have them

Return tool lineages for tools that have them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<::std::collections::HashMap<String, serde_json::Value>>**](map.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version_api_version_get

> serde_json::Value version_api_version_get()
Return Galaxy version information: major/minor version, optional extra info

Return Galaxy version information: major/minor version, optional extra info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version_api_version_get_0

> serde_json::Value version_api_version_get_0()
Return Galaxy version information: major/minor version, optional extra info

Return Galaxy version information: major/minor version, optional extra info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## whoami_api_whoami_get

> crate::models::UserModel whoami_api_whoami_get(run_as)
Return information about the current authenticated user

Return information about the current authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::UserModel**](UserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## whoami_api_whoami_get_0

> crate::models::UserModel whoami_api_whoami_get_0(run_as)
Return information about the current authenticated user

Return information about the current authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::UserModel**](UserModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

