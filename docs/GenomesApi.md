# \GenomesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_api_genomes_get**](GenomesApi.md#index_api_genomes_get) | **GET** /api/genomes | Return a list of installed genomes
[**index_api_genomes_get_0**](GenomesApi.md#index_api_genomes_get_0) | **GET** /api/genomes | Return a list of installed genomes
[**indexes_api_genomes_id_indexes_get**](GenomesApi.md#indexes_api_genomes_id_indexes_get) | **GET** /api/genomes/{id}/indexes | Return all available indexes for a genome id for provided type
[**indexes_api_genomes_id_indexes_get_0**](GenomesApi.md#indexes_api_genomes_id_indexes_get_0) | **GET** /api/genomes/{id}/indexes | Return all available indexes for a genome id for provided type
[**sequences_api_genomes_id_sequences_get**](GenomesApi.md#sequences_api_genomes_id_sequences_get) | **GET** /api/genomes/{id}/sequences | Return raw sequence data
[**sequences_api_genomes_id_sequences_get_0**](GenomesApi.md#sequences_api_genomes_id_sequences_get_0) | **GET** /api/genomes/{id}/sequences | Return raw sequence data
[**show_api_genomes_id_get**](GenomesApi.md#show_api_genomes_id_get) | **GET** /api/genomes/{id} | Return information about build <id>
[**show_api_genomes_id_get_0**](GenomesApi.md#show_api_genomes_id_get_0) | **GET** /api/genomes/{id} | Return information about build <id>



## index_api_genomes_get

> Vec<Vec<String>> index_api_genomes_get(chrom_info, run_as)
Return a list of installed genomes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chrom_info** | Option<**bool**> | If true, return genome keys with chromosome lengths |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<Vec<String>>**](array.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_genomes_get_0

> Vec<Vec<String>> index_api_genomes_get_0(chrom_info, run_as)
Return a list of installed genomes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chrom_info** | Option<**bool**> | If true, return genome keys with chromosome lengths |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<Vec<String>>**](array.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indexes_api_genomes_id_indexes_get

> serde_json::Value indexes_api_genomes_id_indexes_get(id, r#type, format)
Return all available indexes for a genome id for provided type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**r#type** | Option<**String**> | Index type |  |[default to fasta_indexes]
**format** | Option<**String**> | Format |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indexes_api_genomes_id_indexes_get_0

> serde_json::Value indexes_api_genomes_id_indexes_get_0(id, r#type, format)
Return all available indexes for a genome id for provided type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**r#type** | Option<**String**> | Index type |  |[default to fasta_indexes]
**format** | Option<**String**> | Format |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sequences_api_genomes_id_sequences_get

> serde_json::Value sequences_api_genomes_id_sequences_get(id, reference, chrom, low, high, format, run_as)
Return raw sequence data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**reference** | Option<**bool**> | If true, return reference data |  |
**chrom** | Option<**String**> | Limits size of returned data |  |
**low** | Option<**i32**> | Limits size of returned data |  |
**high** | Option<**i32**> | Limits size of returned data |  |
**format** | Option<**String**> | Format |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sequences_api_genomes_id_sequences_get_0

> serde_json::Value sequences_api_genomes_id_sequences_get_0(id, reference, chrom, low, high, format, run_as)
Return raw sequence data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**reference** | Option<**bool**> | If true, return reference data |  |
**chrom** | Option<**String**> | Limits size of returned data |  |
**low** | Option<**i32**> | Limits size of returned data |  |
**high** | Option<**i32**> | Limits size of returned data |  |
**format** | Option<**String**> | Format |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_genomes_id_get

> serde_json::Value show_api_genomes_id_get(id, reference, num, chrom, low, high, format, run_as)
Return information about build <id>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**reference** | Option<**bool**> | If true, return reference data |  |
**num** | Option<**i32**> | Limits size of returned data |  |
**chrom** | Option<**String**> | Limits size of returned data |  |
**low** | Option<**i32**> | Limits size of returned data |  |
**high** | Option<**i32**> | Limits size of returned data |  |
**format** | Option<**String**> | Format |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_genomes_id_get_0

> serde_json::Value show_api_genomes_id_get_0(id, reference, num, chrom, low, high, format, run_as)
Return information about build <id>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genome ID | [required] |
**reference** | Option<**bool**> | If true, return reference data |  |
**num** | Option<**i32**> | Limits size of returned data |  |
**chrom** | Option<**String**> | Limits size of returned data |  |
**low** | Option<**i32**> | Limits size of returned data |  |
**high** | Option<**i32**> | Limits size of returned data |  |
**format** | Option<**String**> | Format |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

