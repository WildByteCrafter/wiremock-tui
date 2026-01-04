# \NearMissesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_near_misses_for_request**](NearMissesApi.md#find_near_misses_for_request) | **POST** /__admin/near-misses/request | Find near misses matching specific request
[**find_near_misses_for_request_pattern**](NearMissesApi.md#find_near_misses_for_request_pattern) | **POST** /__admin/near-misses/request-pattern | Find near misses matching request pattern
[**retrieve_near_misses_for_unmatched_requests**](NearMissesApi.md#retrieve_near_misses_for_unmatched_requests) | **GET** /__admin/requests/unmatched/near-misses | 



## find_near_misses_for_request

> models::RetrieveNearMissesForUnmatchedRequests200Response find_near_misses_for_request(logged_request)
Find near misses matching specific request

Find at most 3 near misses for closest stub mappings to the specified request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logged_request** | [**LoggedRequest**](LoggedRequest.md) |  | [required] |

### Return type

[**models::RetrieveNearMissesForUnmatchedRequests200Response**](retrieveNearMissesForUnmatchedRequests_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_near_misses_for_request_pattern

> models::RetrieveNearMissesForUnmatchedRequests200Response find_near_misses_for_request_pattern(request_pattern)
Find near misses matching request pattern

Find at most 3 near misses for closest logged requests to the specified request pattern

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_pattern** | [**RequestPattern**](RequestPattern.md) |  | [required] |

### Return type

[**models::RetrieveNearMissesForUnmatchedRequests200Response**](retrieveNearMissesForUnmatchedRequests_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_near_misses_for_unmatched_requests

> models::RetrieveNearMissesForUnmatchedRequests200Response retrieve_near_misses_for_unmatched_requests()


Retrieve near-misses for all unmatched requests

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RetrieveNearMissesForUnmatchedRequests200Response**](retrieveNearMissesForUnmatchedRequests_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

