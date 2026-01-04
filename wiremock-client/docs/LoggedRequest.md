# LoggedRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The unique identifier for this request | [optional]
**method** | Option<**String**> | The HTTP request method | [optional]
**url** | Option<**String**> | The path and query to match exactly against | [optional]
**absolute_url** | Option<**String**> | The full URL to match against | [optional]
**scheme** | Option<**String**> | The URL scheme (http/https) | [optional]
**host** | Option<**String**> | The host part of the URL | [optional]
**port** | Option<**i32**> | The port number | [optional]
**client_ip** | Option<**String**> | The client IP address | [optional]
**headers** | Option<[**std::collections::HashMap<String, models::HeadersValue>**](headers_value.md)> | HTTP headers | [optional]
**cookies** | Option<[**std::collections::HashMap<String, models::LoggedRequestCookiesValue>**](logged_request_cookies_value.md)> | Cookies received with the request | [optional]
**body** | Option<**String**> | Body string to match against | [optional]
**body_as_base64** | Option<**String**> | Base64 encoded body content | [optional]
**browser_proxy_request** | Option<**bool**> | Whether this request was made via a browser proxy | [optional]
**logged_date** | Option<**i64**> | The timestamp when the request was logged (epoch millis) | [optional]
**logged_date_string** | Option<**String**> | The formatted date string when the request was logged | [optional]
**query_params** | Option<[**std::collections::HashMap<String, models::HeadersValue>**](headers_value.md)> | Query parameters parsed from the URL | [optional]
**form_params** | Option<[**std::collections::HashMap<String, models::HeadersValue>**](headers_value.md)> | Form parameters parsed from the request body | [optional]
**multiparts** | Option<[**Vec<models::LoggedRequestMultipartsInner>**](logged_request_multiparts_inner.md)> | Multipart form data parts | [optional]
**protocol** | Option<**String**> | The HTTP protocol version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


