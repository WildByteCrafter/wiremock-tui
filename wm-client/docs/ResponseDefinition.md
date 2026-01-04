# ResponseDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**i32**> | The HTTP status code to be returned | [optional]
**status_message** | Option<**String**> | The HTTP status message to be returned | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> | Map of response headers to send | [optional]
**additional_proxy_request_headers** | Option<**std::collections::HashMap<String, String>**> | Extra request headers to send when proxying to another host. | [optional]
**remove_proxy_request_headers** | Option<**Vec<String>**> | Request headers to remove when proxying to another host. | [optional]
**body** | Option<**String**> | The response body as a string. Only one of body, base64Body, jsonBody or bodyFileName may be specified. | [optional]
**base64_body** | Option<**String**> | A base64 encoded string used to describe binary data. | [optional]
**json_body** | Option<[**models::ResponseDefinitionAllOfJsonBody**](response_definition_allOf_jsonBody.md)> |  | [optional]
**body_file_name** | Option<**String**> | The path to the file containing the response body, relative to the configured file root. Only one of body, base64Body, jsonBody or bodyFileName may be specified. | [optional]
**fault** | Option<**String**> | The fault to apply (instead of a full, valid response). | [optional]
**fixed_delay_milliseconds** | Option<**i32**> | Number of milliseconds to delay be before sending the response. | [optional]
**delay_distribution** | Option<[**models::DelayDistribution**](delay-distribution.md)> |  | [optional]
**chunked_dribble_delay** | Option<[**models::ResponseDefinitionAllOfChunkedDribbleDelay**](response_definition_allOf_chunkedDribbleDelay.md)> |  | [optional]
**from_configured_stub** | Option<**bool**> | Read-only flag indicating false if this was the default, unmatched response. Not present otherwise. | [optional]
**proxy_base_url** | Option<**String**> | The base URL of the target to proxy matching requests to. | [optional]
**proxy_url_prefix_to_remove** | Option<**String**> | A path segment to remove from the beginning in incoming request URL paths before proxying to the target. | [optional]
**transformer_parameters** | Option<[**serde_json::Value**](.md)> | Parameters to apply to response transformers. | [optional]
**transformers** | Option<**Vec<String>**> | List of names of transformers to apply to this response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


