# \JobsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_api_jobs_get**](JobsApi.md#index_api_jobs_get) | **GET** /api/jobs | Index
[**index_api_jobs_get_0**](JobsApi.md#index_api_jobs_get_0) | **GET** /api/jobs | Index
[**show_api_jobs_id_get**](JobsApi.md#show_api_jobs_id_get) | **GET** /api/jobs/{id} | Show
[**show_api_jobs_id_get_0**](JobsApi.md#show_api_jobs_id_get_0) | **GET** /api/jobs/{id} | Show



## index_api_jobs_get

> Vec<serde_json::Value> index_api_jobs_get(user_details, user_id, view, date_range_min, date_range_max, history_id, workflow_id, invocation_id, order_by, search, limit, offset, state, tool_id, tool_id_like, run_as)
Index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_details** | Option<**bool**> | If true, and requestor is an admin, will return external job id and user email. This is only available to admins. |  |[default to false]
**user_id** | Option<**String**> | an encoded user id to restrict query to, must be own id if not admin user |  |
**view** | Option<[**crate::models::JobIndexViewEnum**](.md)> | Determines columns to return. Defaults to 'collection'. |  |
**date_range_min** | Option<[**DateRangeMinimum**](.md)> | Limit listing of jobs to those that are updated after specified date (e.g. '2014-01-01') |  |
**date_range_max** | Option<[**DateRangeMaximum**](.md)> | Limit listing of jobs to those that are updated before specified date (e.g. '2014-01-01') |  |
**history_id** | Option<**String**> | Limit listing of jobs to those that match the history_id. If none, jobs from any history may be returned. |  |
**workflow_id** | Option<**String**> | Limit listing of jobs to those that match the specified workflow ID. If none, jobs from any workflow (or from no workflows) may be returned. |  |
**invocation_id** | Option<**String**> | Limit listing of jobs to those that match the specified workflow invocation ID. If none, jobs from any workflow invocation (or from no workflows) may be returned. |  |
**order_by** | Option<[**crate::models::JobIndexSortByEnum**](.md)> | Sort results by specified field. |  |
**search** | Option<**String**> | A mix of free text and GitHub-style tags used to filter the index operation.  ## Query Structure  GitHub-style filter tags (not be confused with Galaxy tags) are tags of the form `<tag_name>:<text_no_spaces>` or `<tag_name>:'<text with potential spaces>'`. The tag name *generally* (but not exclusively) corresponds to the name of an attribute on the model being indexed (i.e. a column in the database).  If the tag is quoted, the attribute will be filtered exactly. If the tag is unquoted, generally a partial match will be used to filter the query (i.e. in terms of the implementation this means the database operation `ILIKE` will typically be used).  Once the tagged filters are extracted from the search query, the remaing text is just used to search various documented attributes of the object.  ## GitHub-style Tags Available  `user` : The user email of the user that executed the Job. (The tag `u` can be used a short hand alias for this tag to filter on this attribute.)  `tool_id` : The tool ID corresponding to the job. (The tag `t` can be used a short hand alias for this tag to filter on this attribute.)  `runner` : The job runner name used to execte the job. (The tag `r` can be used a short hand alias for this tag to filter on this attribute.) This tag is only available for requests using admin keys and/or sessions.  `handler` : The job handler name used to execute the job. (The tag `h` can be used a short hand alias for this tag to filter on this attribute.) This tag is only available for requests using admin keys and/or sessions.  ## Free Text  Free text search terms will be searched against the following attributes of the Jobs: `user`, `tool`, `handler`, `runner`.   |  |
**limit** | Option<**i32**> | Maximum number of jobs to return. |  |[default to 500]
**offset** | Option<**i32**> | Return jobs starting from this specified position. For example, if ``limit`` is set to 100 and ``offset`` to 200, jobs 200-299 will be returned. |  |[default to 0]
**state** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of states to filter job query on. If unspecified, jobs of any state may be returned. |  |
**tool_id** | Option<[**Vec<String>**](String.md)> | Limit listing of jobs to those that match one of the included tool_ids. If none, all are returned |  |
**tool_id_like** | Option<[**Vec<String>**](String.md)> | Limit listing of jobs to those that match one of the included tool ID sql-like patterns. If none, all are returned |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_jobs_get_0

> Vec<serde_json::Value> index_api_jobs_get_0(user_details, user_id, view, date_range_min, date_range_max, history_id, workflow_id, invocation_id, order_by, search, limit, offset, state, tool_id, tool_id_like, run_as)
Index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_details** | Option<**bool**> | If true, and requestor is an admin, will return external job id and user email. This is only available to admins. |  |[default to false]
**user_id** | Option<**String**> | an encoded user id to restrict query to, must be own id if not admin user |  |
**view** | Option<[**crate::models::JobIndexViewEnum**](.md)> | Determines columns to return. Defaults to 'collection'. |  |
**date_range_min** | Option<[**DateRangeMinimum**](.md)> | Limit listing of jobs to those that are updated after specified date (e.g. '2014-01-01') |  |
**date_range_max** | Option<[**DateRangeMaximum**](.md)> | Limit listing of jobs to those that are updated before specified date (e.g. '2014-01-01') |  |
**history_id** | Option<**String**> | Limit listing of jobs to those that match the history_id. If none, jobs from any history may be returned. |  |
**workflow_id** | Option<**String**> | Limit listing of jobs to those that match the specified workflow ID. If none, jobs from any workflow (or from no workflows) may be returned. |  |
**invocation_id** | Option<**String**> | Limit listing of jobs to those that match the specified workflow invocation ID. If none, jobs from any workflow invocation (or from no workflows) may be returned. |  |
**order_by** | Option<[**crate::models::JobIndexSortByEnum**](.md)> | Sort results by specified field. |  |
**search** | Option<**String**> | A mix of free text and GitHub-style tags used to filter the index operation.  ## Query Structure  GitHub-style filter tags (not be confused with Galaxy tags) are tags of the form `<tag_name>:<text_no_spaces>` or `<tag_name>:'<text with potential spaces>'`. The tag name *generally* (but not exclusively) corresponds to the name of an attribute on the model being indexed (i.e. a column in the database).  If the tag is quoted, the attribute will be filtered exactly. If the tag is unquoted, generally a partial match will be used to filter the query (i.e. in terms of the implementation this means the database operation `ILIKE` will typically be used).  Once the tagged filters are extracted from the search query, the remaing text is just used to search various documented attributes of the object.  ## GitHub-style Tags Available  `user` : The user email of the user that executed the Job. (The tag `u` can be used a short hand alias for this tag to filter on this attribute.)  `tool_id` : The tool ID corresponding to the job. (The tag `t` can be used a short hand alias for this tag to filter on this attribute.)  `runner` : The job runner name used to execte the job. (The tag `r` can be used a short hand alias for this tag to filter on this attribute.) This tag is only available for requests using admin keys and/or sessions.  `handler` : The job handler name used to execute the job. (The tag `h` can be used a short hand alias for this tag to filter on this attribute.) This tag is only available for requests using admin keys and/or sessions.  ## Free Text  Free text search terms will be searched against the following attributes of the Jobs: `user`, `tool`, `handler`, `runner`.   |  |
**limit** | Option<**i32**> | Maximum number of jobs to return. |  |[default to 500]
**offset** | Option<**i32**> | Return jobs starting from this specified position. For example, if ``limit`` is set to 100 and ``offset`` to 200, jobs 200-299 will be returned. |  |[default to 0]
**state** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of states to filter job query on. If unspecified, jobs of any state may be returned. |  |
**tool_id** | Option<[**Vec<String>**](String.md)> | Limit listing of jobs to those that match one of the included tool_ids. If none, all are returned |  |
**tool_id_like** | Option<[**Vec<String>**](String.md)> | Limit listing of jobs to those that match one of the included tool ID sql-like patterns. If none, all are returned |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_jobs_id_get

> serde_json::Value show_api_jobs_id_get(id, full, run_as)
Show

Return dictionary containing description of job data  Parameters - id: ID of job to return - full: Return extra information ?

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**full** | Option<**bool**> |  |  |[default to false]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_jobs_id_get_0

> serde_json::Value show_api_jobs_id_get_0(id, full, run_as)
Show

Return dictionary containing description of job data  Parameters - id: ID of job to return - full: Return extra information ?

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**full** | Option<**bool**> |  |  |[default to false]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

