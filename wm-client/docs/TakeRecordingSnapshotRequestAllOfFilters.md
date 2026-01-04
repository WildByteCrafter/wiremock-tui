# TakeRecordingSnapshotRequestAllOfFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheme** | Option<**String**> | The scheme (protocol) part of the request URL | [optional]
**host** | Option<**String**> | The hostname part of the request URL | [optional]
**port** | Option<**i32**> | The HTTP port number of the request URL | [optional]
**method** | Option<**String**> | The HTTP request method e.g. GET | [optional]
**url** | Option<**String**> | The path and query to match exactly against. Only one of url, urlPattern, urlPath or urlPathPattern may be specified. | [optional]
**url_path** | Option<**String**> | The path to match exactly against. Only one of url, urlPattern, urlPath or urlPathPattern may be specified. | [optional]
**url_path_pattern** | Option<**String**> | The path regex to match against. Only one of url, urlPattern, urlPath or urlPathPattern may be specified. | [optional]
**url_pattern** | Option<**String**> | The path and query regex to match against. Only one of url, urlPattern, urlPath or urlPathPattern may be specified. | [optional]
**url_path_template** | Option<**String**> | The path template to match against. Must conform to the OpenAPI compatible subset of the RFC 6570 URI Template specification. Only one of url, urlPattern, urlPath or urlPathPattern may be specified.  | [optional]
**path_parameters** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | Path parameter patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form. Can only be used when the urlPathPattern URL match type is in use and all keys must be present as variables in the path template. | [optional]
**query_parameters** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | Query parameter patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form | [optional]
**form_parameters** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | application/x-www-form-urlencoded form parameter patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form | [optional]
**headers** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | Header patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form | [optional]
**client_ip** | Option<**String**> | The client IP address to match against | [optional]
**basic_auth_credentials** | Option<[**models::RequestPatternBasicAuthCredentials**](request_pattern_basicAuthCredentials.md)> |  | [optional]
**cookies** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | Cookie patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form | [optional]
**body_patterns** | Option<[**Vec<models::ContentPattern>**](content-pattern.md)> | Request body patterns to match against in the { \"<predicate>\": \"<value>\" } form | [optional]
**custom_matcher** | Option<[**models::RequestPatternCustomMatcher**](request_pattern_customMatcher.md)> |  | [optional]
**multipart_patterns** | Option<[**Vec<models::RequestPatternMultipartPatternsInner>**](request_pattern_multipartPatterns_inner.md)> | Multipart patterns to match against headers and body. | [optional]
**ids** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


