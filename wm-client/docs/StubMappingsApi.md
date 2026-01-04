# \StubMappingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_stub_mapping**](StubMappingsApi.md#create_new_stub_mapping) | **POST** /__admin/mappings | Create a new stub mapping
[**delete_all_stub_mappings**](StubMappingsApi.md#delete_all_stub_mappings) | **DELETE** /__admin/mappings | Delete all stub mappings
[**delete_stub_mapping**](StubMappingsApi.md#delete_stub_mapping) | **DELETE** /__admin/mappings/{stubMappingId} | Delete a stub mapping
[**find_stub_mappings_by_metadata**](StubMappingsApi.md#find_stub_mappings_by_metadata) | **POST** /__admin/mappings/find-by-metadata | 
[**find_unmatched_stub_mappings**](StubMappingsApi.md#find_unmatched_stub_mappings) | **GET** /__admin/mappings/unmatched | Find unmatched stub mappings
[**get_all_stub_mappings**](StubMappingsApi.md#get_all_stub_mappings) | **GET** /__admin/mappings | Get all stub mappings
[**get_stub_mapping_by_id**](StubMappingsApi.md#get_stub_mapping_by_id) | **GET** /__admin/mappings/{stubMappingId} | Get stub mapping by ID
[**import_stub_mappings**](StubMappingsApi.md#import_stub_mappings) | **POST** /__admin/mappings/import | Import stub mappings
[**persist_stub_mappings**](StubMappingsApi.md#persist_stub_mappings) | **POST** /__admin/mappings/save | Persist stub mappings
[**remove_stub_mappings_by_metadata**](StubMappingsApi.md#remove_stub_mappings_by_metadata) | **POST** /__admin/mappings/remove-by-metadata | Delete stub mappings matching metadata
[**remove_unmatched_stub_mappings**](StubMappingsApi.md#remove_unmatched_stub_mappings) | **DELETE** /__admin/mappings/unmatched | Remove unmatched stub mappings
[**reset_stub_mappings**](StubMappingsApi.md#reset_stub_mappings) | **POST** /__admin/mappings/reset | Reset stub mappings
[**update_stub_mapping**](StubMappingsApi.md#update_stub_mapping) | **PUT** /__admin/mappings/{stubMappingId} | Update a stub mapping



## create_new_stub_mapping

> models::StubMapping create_new_stub_mapping(stub_mapping)
Create a new stub mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stub_mapping** | Option<[**StubMapping**](StubMapping.md)> |  |  |

### Return type

[**models::StubMapping**](stub-mapping.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_stub_mappings

> delete_all_stub_mappings()
Delete all stub mappings

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


## delete_stub_mapping

> delete_stub_mapping(stub_mapping_id)
Delete a stub mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stub_mapping_id** | **String** | The UUID of stub mapping | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_stub_mappings_by_metadata

> models::StubMappings find_stub_mappings_by_metadata(content_pattern)


Find stubs by matching on their metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_pattern** | [**ContentPattern**](ContentPattern.md) |  | [required] |

### Return type

[**models::StubMappings**](stub-mappings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_unmatched_stub_mappings

> models::StubMappings find_unmatched_stub_mappings()
Find unmatched stub mappings

Find stub mappings that haven't matched any requests in the journal

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StubMappings**](stub-mappings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_stub_mappings

> models::StubMappings get_all_stub_mappings(limit, offset)
Get all stub mappings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of results to return |  |
**offset** | Option<**i32**> | The start index of the results to return |  |

### Return type

[**models::StubMappings**](stub-mappings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stub_mapping_by_id

> models::StubMapping get_stub_mapping_by_id(stub_mapping_id)
Get stub mapping by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stub_mapping_id** | **String** | The UUID of stub mapping | [required] |

### Return type

[**models::StubMapping**](stub-mapping.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_stub_mappings

> import_stub_mappings()
Import stub mappings

Import given stub mappings to the backing store

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


## persist_stub_mappings

> persist_stub_mappings()
Persist stub mappings

Save all persistent stub mappings to the backing store

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


## remove_stub_mappings_by_metadata

> remove_stub_mappings_by_metadata(content_pattern)
Delete stub mappings matching metadata

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_unmatched_stub_mappings

> remove_unmatched_stub_mappings()
Remove unmatched stub mappings

Remove stub mappings that haven't matched any requests in the journal

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


## reset_stub_mappings

> reset_stub_mappings()
Reset stub mappings

Restores stub mappings to the defaults defined back in the backing store

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


## update_stub_mapping

> models::StubMapping update_stub_mapping(stub_mapping_id, stub_mapping)
Update a stub mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stub_mapping_id** | **String** | The UUID of stub mapping | [required] |
**stub_mapping** | Option<[**StubMapping**](StubMapping.md)> |  |  |

### Return type

[**models::StubMapping**](stub-mapping.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

