# StubMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | This stub mapping's unique identifier | [optional]
**uuid** | Option<**String**> | Alias for the id | [optional]
**name** | Option<**String**> | The stub mapping's name | [optional]
**request** | Option<[**models::RequestPattern**](request-pattern.md)> |  | [optional]
**response** | Option<[**models::ResponseDefinition**](response-definition.md)> |  | [optional]
**persistent** | Option<**bool**> | Indicates that the stub mapping should be persisted immediately on create/update/delete and survive resets to default. | [optional]
**priority** | Option<**i32**> | This stub mapping's priority relative to others. 1 is highest. | [optional]
**scenario_name** | Option<**String**> | The name of the scenario that this stub mapping is part of | [optional]
**required_scenario_state** | Option<**String**> | The required state of the scenario in order for this stub to be matched. | [optional]
**new_scenario_state** | Option<**String**> | The new state for the scenario to be updated to after this stub is served. | [optional]
**post_serve_actions** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | DEPRECATED: Use `serveEventListeners` instead. A map of the names of post serve action extensions to trigger and their parameters.  | [optional]
**serve_event_listeners** | Option<[**Vec<models::StubMappingServeEventListenersInner>**](stub_mapping_serveEventListeners_inner.md)> | The list of serve event listeners | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary metadata to be used for e.g. tagging, documentation. Can also be used to find and remove stubs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


