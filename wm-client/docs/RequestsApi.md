# \RequestsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**count_requests_by_criteria**](RequestsApi.md#count_requests_by_criteria) | **POST** /__admin/requests/count | Count requests by criteria
[**delete_all_requests_in_journal**](RequestsApi.md#delete_all_requests_in_journal) | **DELETE** /__admin/requests | Delete all requests in journal
[**delete_request_by_id**](RequestsApi.md#delete_request_by_id) | **DELETE** /__admin/requests/{requestId} | Delete request by ID
[**empty_request_journal**](RequestsApi.md#empty_request_journal) | **POST** /__admin/requests/reset | Empty the request journal
[**find_requests_by_criteria**](RequestsApi.md#find_requests_by_criteria) | **POST** /__admin/requests/find | Find requests by criteria
[**find_unmatched_requests**](RequestsApi.md#find_unmatched_requests) | **GET** /__admin/requests/unmatched | Find unmatched requests
[**get_all_requests_in_journal**](RequestsApi.md#get_all_requests_in_journal) | **GET** /__admin/requests | Get all requests in journal
[**get_request_by_id**](RequestsApi.md#get_request_by_id) | **GET** /__admin/requests/{requestId} | Get request by ID
[**remove_requests_by_criteria**](RequestsApi.md#remove_requests_by_criteria) | **POST** /__admin/requests/remove | Remove requests by criteria
[**remove_requests_by_metadata**](RequestsApi.md#remove_requests_by_metadata) | **POST** /__admin/requests/remove-by-metadata | Delete requests mappings matching metadata



## count_requests_by_criteria

> models::CountRequestsByCriteria200Response count_requests_by_criteria(request_pattern)
Count requests by criteria

Count requests logged in the journal matching the specified criteria

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_pattern** | [**RequestPattern**](RequestPattern.md) |  | [required] |

### Return type

[**models::CountRequestsByCriteria200Response**](countRequestsByCriteria_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_requests_in_journal

> delete_all_requests_in_journal()
Delete all requests in journal

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_request_by_id

> delete_request_by_id(request_id)
Delete request by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The UUID of the logged request | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## empty_request_journal

> empty_request_journal()
Empty the request journal

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_requests_by_criteria

> find_requests_by_criteria(request_pattern)
Find requests by criteria

Retrieve details of requests logged in the journal matching the specified criteria

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_pattern** | [**RequestPattern**](RequestPattern.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_unmatched_requests

> find_unmatched_requests()
Find unmatched requests

Get details of logged requests that weren't matched by any stub mapping

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_requests_in_journal

> get_all_requests_in_journal(limit, since)
Get all requests in journal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**String**> | The maximum number of results to return |  |
**since** | Option<**String**> | Only return logged requests after this date |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request_by_id

> get_request_by_id(request_id)
Get request by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The UUID of the logged request | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_requests_by_criteria

> remove_requests_by_criteria(request_pattern)
Remove requests by criteria

Removed requests logged in the journal matching the specified criteria

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_pattern** | [**RequestPattern**](RequestPattern.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_requests_by_metadata

> remove_requests_by_metadata(content_pattern)
Delete requests mappings matching metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_pattern** | Option<[**ContentPattern**](ContentPattern.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

