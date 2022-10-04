# \ToursApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_api_tours_get**](ToursApi.md#index_api_tours_get) | **GET** /api/tours | Index
[**index_api_tours_get_0**](ToursApi.md#index_api_tours_get_0) | **GET** /api/tours | Index
[**show_api_tours_tour_id_get**](ToursApi.md#show_api_tours_tour_id_get) | **GET** /api/tours/{tour_id} | Show
[**show_api_tours_tour_id_get_0**](ToursApi.md#show_api_tours_tour_id_get_0) | **GET** /api/tours/{tour_id} | Show
[**update_tour_api_tours_tour_id_post**](ToursApi.md#update_tour_api_tours_tour_id_post) | **POST** /api/tours/{tour_id} | Update Tour
[**update_tour_api_tours_tour_id_post_0**](ToursApi.md#update_tour_api_tours_tour_id_post_0) | **POST** /api/tours/{tour_id} | Update Tour



## index_api_tours_get

> Vec<crate::models::Tour> index_api_tours_get()
Index

Return list of available tours.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Tour>**](Tour.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_tours_get_0

> Vec<crate::models::Tour> index_api_tours_get_0()
Index

Return list of available tours.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Tour>**](Tour.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_tours_tour_id_get

> crate::models::TourDetails show_api_tours_tour_id_get(tour_id)
Show

Return a tour definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tour_id** | **String** |  | [required] |

### Return type

[**crate::models::TourDetails**](TourDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_tours_tour_id_get_0

> crate::models::TourDetails show_api_tours_tour_id_get_0(tour_id)
Show

Return a tour definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tour_id** | **String** |  | [required] |

### Return type

[**crate::models::TourDetails**](TourDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tour_api_tours_tour_id_post

> crate::models::TourDetails update_tour_api_tours_tour_id_post(tour_id, run_as)
Update Tour

Return a tour definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tour_id** | **String** |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::TourDetails**](TourDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tour_api_tours_tour_id_post_0

> crate::models::TourDetails update_tour_api_tours_tour_id_post_0(tour_id, run_as)
Update Tour

Return a tour definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tour_id** | **String** |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::TourDetails**](TourDetails.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

