# \DatatypesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**converters_api_datatypes_converters_get**](DatatypesApi.md#converters_api_datatypes_converters_get) | **GET** /api/datatypes/converters | Returns the list of all installed converters
[**converters_api_datatypes_converters_get_0**](DatatypesApi.md#converters_api_datatypes_converters_get_0) | **GET** /api/datatypes/converters | Returns the list of all installed converters
[**edam_data_api_datatypes_edam_data_get**](DatatypesApi.md#edam_data_api_datatypes_edam_data_get) | **GET** /api/datatypes/edam_data | Returns a dictionary/map of datatypes and EDAM data
[**edam_data_api_datatypes_edam_data_get_0**](DatatypesApi.md#edam_data_api_datatypes_edam_data_get_0) | **GET** /api/datatypes/edam_data | Returns a dictionary/map of datatypes and EDAM data
[**edam_formats_api_datatypes_edam_formats_get**](DatatypesApi.md#edam_formats_api_datatypes_edam_formats_get) | **GET** /api/datatypes/edam_formats | Returns a dictionary/map of datatypes and EDAM formats
[**edam_formats_api_datatypes_edam_formats_get_0**](DatatypesApi.md#edam_formats_api_datatypes_edam_formats_get_0) | **GET** /api/datatypes/edam_formats | Returns a dictionary/map of datatypes and EDAM formats
[**index_api_datatypes_get**](DatatypesApi.md#index_api_datatypes_get) | **GET** /api/datatypes | Lists all available data types
[**index_api_datatypes_get_0**](DatatypesApi.md#index_api_datatypes_get_0) | **GET** /api/datatypes | Lists all available data types
[**mapping_api_datatypes_mapping_get**](DatatypesApi.md#mapping_api_datatypes_mapping_get) | **GET** /api/datatypes/mapping | Returns mappings for data types and their implementing classes
[**mapping_api_datatypes_mapping_get_0**](DatatypesApi.md#mapping_api_datatypes_mapping_get_0) | **GET** /api/datatypes/mapping | Returns mappings for data types and their implementing classes
[**sniffers_api_datatypes_sniffers_get**](DatatypesApi.md#sniffers_api_datatypes_sniffers_get) | **GET** /api/datatypes/sniffers | Returns the list of all installed sniffers
[**sniffers_api_datatypes_sniffers_get_0**](DatatypesApi.md#sniffers_api_datatypes_sniffers_get_0) | **GET** /api/datatypes/sniffers | Returns the list of all installed sniffers
[**types_and_mapping_api_datatypes_types_and_mapping_get**](DatatypesApi.md#types_and_mapping_api_datatypes_types_and_mapping_get) | **GET** /api/datatypes/types_and_mapping | Returns all the data types extensions and their mappings
[**types_and_mapping_api_datatypes_types_and_mapping_get_0**](DatatypesApi.md#types_and_mapping_api_datatypes_types_and_mapping_get_0) | **GET** /api/datatypes/types_and_mapping | Returns all the data types extensions and their mappings



## converters_api_datatypes_converters_get

> Vec<crate::models::DatatypeConverter> converters_api_datatypes_converters_get()
Returns the list of all installed converters

Gets the list of all installed converters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DatatypeConverter>**](DatatypeConverter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## converters_api_datatypes_converters_get_0

> Vec<crate::models::DatatypeConverter> converters_api_datatypes_converters_get_0()
Returns the list of all installed converters

Gets the list of all installed converters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DatatypeConverter>**](DatatypeConverter.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edam_data_api_datatypes_edam_data_get

> ::std::collections::HashMap<String, String> edam_data_api_datatypes_edam_data_get()
Returns a dictionary/map of datatypes and EDAM data

Gets a map of datatypes and their corresponding EDAM data.

### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edam_data_api_datatypes_edam_data_get_0

> ::std::collections::HashMap<String, String> edam_data_api_datatypes_edam_data_get_0()
Returns a dictionary/map of datatypes and EDAM data

Gets a map of datatypes and their corresponding EDAM data.

### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edam_formats_api_datatypes_edam_formats_get

> ::std::collections::HashMap<String, String> edam_formats_api_datatypes_edam_formats_get()
Returns a dictionary/map of datatypes and EDAM formats

Gets a map of datatypes and their corresponding EDAM formats.

### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edam_formats_api_datatypes_edam_formats_get_0

> ::std::collections::HashMap<String, String> edam_formats_api_datatypes_edam_formats_get_0()
Returns a dictionary/map of datatypes and EDAM formats

Gets a map of datatypes and their corresponding EDAM formats.

### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_datatypes_get

> crate::models::ResponseIndexApiDatatypesGet index_api_datatypes_get(extension_only, upload_only)
Lists all available data types

Gets the list of all available data types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_only** | Option<**bool**> | Whether to return only the datatype's extension rather than the datatype's details |  |[default to true]
**upload_only** | Option<**bool**> | Whether to return only datatypes which can be uploaded |  |[default to true]

### Return type

[**crate::models::ResponseIndexApiDatatypesGet**](Response_Index_Api_Datatypes_Get.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_datatypes_get_0

> crate::models::ResponseIndexApiDatatypesGet index_api_datatypes_get_0(extension_only, upload_only)
Lists all available data types

Gets the list of all available data types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_only** | Option<**bool**> | Whether to return only the datatype's extension rather than the datatype's details |  |[default to true]
**upload_only** | Option<**bool**> | Whether to return only datatypes which can be uploaded |  |[default to true]

### Return type

[**crate::models::ResponseIndexApiDatatypesGet**](Response_Index_Api_Datatypes_Get.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mapping_api_datatypes_mapping_get

> crate::models::DatatypesMap mapping_api_datatypes_mapping_get()
Returns mappings for data types and their implementing classes

Gets mappings for data types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DatatypesMap**](DatatypesMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mapping_api_datatypes_mapping_get_0

> crate::models::DatatypesMap mapping_api_datatypes_mapping_get_0()
Returns mappings for data types and their implementing classes

Gets mappings for data types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DatatypesMap**](DatatypesMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sniffers_api_datatypes_sniffers_get

> Vec<String> sniffers_api_datatypes_sniffers_get()
Returns the list of all installed sniffers

Gets the list of all installed data type sniffers.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sniffers_api_datatypes_sniffers_get_0

> Vec<String> sniffers_api_datatypes_sniffers_get_0()
Returns the list of all installed sniffers

Gets the list of all installed data type sniffers.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## types_and_mapping_api_datatypes_types_and_mapping_get

> crate::models::DatatypesCombinedMap types_and_mapping_api_datatypes_types_and_mapping_get(extension_only, upload_only)
Returns all the data types extensions and their mappings

Combines the datatype information from (/api/datatypes) and the mapping information from (/api/datatypes/mapping) into a single response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_only** | Option<**bool**> | Whether to return only the datatype's extension rather than the datatype's details |  |[default to true]
**upload_only** | Option<**bool**> | Whether to return only datatypes which can be uploaded |  |[default to true]

### Return type

[**crate::models::DatatypesCombinedMap**](DatatypesCombinedMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## types_and_mapping_api_datatypes_types_and_mapping_get_0

> crate::models::DatatypesCombinedMap types_and_mapping_api_datatypes_types_and_mapping_get_0(extension_only, upload_only)
Returns all the data types extensions and their mappings

Combines the datatype information from (/api/datatypes) and the mapping information from (/api/datatypes/mapping) into a single response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_only** | Option<**bool**> | Whether to return only the datatype's extension rather than the datatype's details |  |[default to true]
**upload_only** | Option<**bool**> | Whether to return only datatypes which can be uploaded |  |[default to true]

### Return type

[**crate::models::DatatypesCombinedMap**](DatatypesCombinedMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

