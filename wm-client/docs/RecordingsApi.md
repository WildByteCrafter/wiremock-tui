# \RecordingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_recording_status**](RecordingsApi.md#get_recording_status) | **GET** /__admin/recordings/status | Get recording status
[**start_recording**](RecordingsApi.md#start_recording) | **POST** /__admin/recordings/start | Start recording
[**stop_recording**](RecordingsApi.md#stop_recording) | **POST** /__admin/recordings/stop | Stop recording
[**take_recording_snapshot**](RecordingsApi.md#take_recording_snapshot) | **POST** /__admin/recordings/snapshot | Take a snapshot recording



## get_recording_status

> models::GetRecordingStatus200Response get_recording_status()
Get recording status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetRecordingStatus200Response**](getRecordingStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_recording

> start_recording(start_recording_request)
Start recording

Begin recording stub mappings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_recording_request** | [**StartRecordingRequest**](StartRecordingRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_recording

> models::StubMappings stop_recording()
Stop recording

End recording of stub mappings

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


## take_recording_snapshot

> models::StubMappings take_recording_snapshot(take_recording_snapshot_request)
Take a snapshot recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**take_recording_snapshot_request** | [**TakeRecordingSnapshotRequest**](TakeRecordingSnapshotRequest.md) |  | [required] |

### Return type

[**models::StubMappings**](stub-mappings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

