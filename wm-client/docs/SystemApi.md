# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_health**](SystemApi.md#get_health) | **GET** /__admin/health | Return the health of the WireMock server
[**get_version**](SystemApi.md#get_version) | **GET** /__admin/version | Return the version of the WireMock server
[**reset_mappings_and_journal**](SystemApi.md#reset_mappings_and_journal) | **POST** /__admin/reset | Reset mappings and request journal
[**shutdown_server**](SystemApi.md#shutdown_server) | **POST** /__admin/shutdown | Shutdown the WireMock server
[**update_global_settings**](SystemApi.md#update_global_settings) | **POST** /__admin/settings | Update global settings



## get_health

> models::Health get_health()
Return the health of the WireMock server

Returns the health of the WireMock server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Health**](health.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> models::GetVersion200Response get_version()
Return the version of the WireMock server

Returns the version of the WireMock server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetVersion200Response**](getVersion_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_mappings_and_journal

> reset_mappings_and_journal()
Reset mappings and request journal

Reset mappings to the default state and reset the request journal

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


## shutdown_server

> shutdown_server()
Shutdown the WireMock server

Shutdown the WireMock server

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


## update_global_settings

> update_global_settings(update_global_settings_request)
Update global settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_global_settings_request** | [**UpdateGlobalSettingsRequest**](UpdateGlobalSettingsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

