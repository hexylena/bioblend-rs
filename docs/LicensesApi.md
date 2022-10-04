# \LicensesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_licenses_id_get**](LicensesApi.md#get_api_licenses_id_get) | **GET** /api/licenses/{id} | Gets the SPDX license metadata associated with the short identifier
[**get_api_licenses_id_get_0**](LicensesApi.md#get_api_licenses_id_get_0) | **GET** /api/licenses/{id} | Gets the SPDX license metadata associated with the short identifier
[**index_api_licenses_get**](LicensesApi.md#index_api_licenses_get) | **GET** /api/licenses | Lists all available SPDX licenses
[**index_api_licenses_get_0**](LicensesApi.md#index_api_licenses_get_0) | **GET** /api/licenses | Lists all available SPDX licenses



## get_api_licenses_id_get

> crate::models::LicenseMetadataModel get_api_licenses_id_get(id)
Gets the SPDX license metadata associated with the short identifier

Returns the license metadata associated with the given [SPDX license short ID](https://spdx.github.io/spdx-spec/appendix-I-SPDX-license-list/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The [SPDX license short identifier](https://spdx.github.io/spdx-spec/appendix-I-SPDX-license-list/) | [required] |

### Return type

[**crate::models::LicenseMetadataModel**](LicenseMetadataModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_licenses_id_get_0

> crate::models::LicenseMetadataModel get_api_licenses_id_get_0(id)
Gets the SPDX license metadata associated with the short identifier

Returns the license metadata associated with the given [SPDX license short ID](https://spdx.github.io/spdx-spec/appendix-I-SPDX-license-list/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | The [SPDX license short identifier](https://spdx.github.io/spdx-spec/appendix-I-SPDX-license-list/) | [required] |

### Return type

[**crate::models::LicenseMetadataModel**](LicenseMetadataModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_licenses_get

> Vec<crate::models::LicenseMetadataModel> index_api_licenses_get()
Lists all available SPDX licenses

Returns an index with all the available [SPDX licenses](https://spdx.org/licenses/).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LicenseMetadataModel>**](LicenseMetadataModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_licenses_get_0

> Vec<crate::models::LicenseMetadataModel> index_api_licenses_get_0()
Lists all available SPDX licenses

Returns an index with all the available [SPDX licenses](https://spdx.org/licenses/).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LicenseMetadataModel>**](LicenseMetadataModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

