# StartRecordingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capture_headers** | Option<[**std::collections::HashMap<String, models::RecordSpecCaptureHeadersValue>**](record_spec_captureHeaders_value.md)> | Headers from the request to include in the generated stub mappings, mapped to parameter objects. The only parameter available is \"caseInsensitive\", which defaults to false | [optional]
**extract_body_criteria** | Option<[**models::RecordSpecExtractBodyCriteria**](record_spec_extractBodyCriteria.md)> |  | [optional]
**persist** | Option<**bool**> | Whether to save stub mappings to the file system or just return them | [optional][default to true]
**repeats_as_scenarios** | Option<**bool**> | When true, duplicate requests will be added to a Scenario. When false, duplicates are discarded | [optional][default to true]
**request_body_pattern** | Option<[**models::RecordSpecRequestBodyPattern**](record_spec_requestBodyPattern.md)> |  | [optional]
**transformer_parameters** | Option<[**serde_json::Value**](.md)> | List of names of stub mappings transformers to apply to generated stubs | [optional]
**transformers** | Option<**Vec<String>**> | Parameters to pass to stub mapping transformers | [optional]
**filters** | Option<[**models::RequestPattern**](request-pattern.md)> | Filter requests for which to create stub mapping | [optional]
**target_base_url** | Option<**String**> | Target URL when using the record and playback API | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


