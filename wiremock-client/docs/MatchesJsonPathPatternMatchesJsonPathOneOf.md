# MatchesJsonPathPatternMatchesJsonPathOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**equal_to** | **String** |  | 
**case_insensitive** | Option<**bool**> |  | [optional]
**binary_equal_to** | **String** | A base64 encoded string used to describe binary data. | 
**contains** | **String** |  | 
**does_not_contain** | **String** |  | 
**matches** | **String** |  | 
**does_not_match** | **String** |  | 
**not** | [**models::ContentPattern**](content-pattern.md) |  | 
**before** | **String** |  | 
**actual_format** | Option<**String**> |  | [optional]
**truncate_expected** | Option<[**models::Truncation**](truncation.md)> |  | [optional]
**truncate_actual** | Option<[**models::Truncation**](truncation.md)> |  | [optional]
**after** | **String** |  | 
**equal_to_date_time** | **String** |  | 
**equal_to_json** | [**models::EqualToJsonPatternEqualToJson**](equal_to_json_pattern_equalToJson.md) |  | 
**ignore_extra_elements** | Option<**bool**> |  | [optional]
**ignore_array_order** | Option<**bool**> |  | [optional]
**matches_json_path** | [**models::MatchesJsonPathPatternMatchesJsonPath**](matches_json_path_pattern_matchesJsonPath.md) |  | 
**equal_to_xml** | **String** |  | 
**enable_placeholders** | Option<**bool**> |  | [optional]
**placeholder_opening_delimiter_regex** | Option<**String**> |  | [optional]
**placeholder_closing_delimiter_regex** | Option<**String**> |  | [optional]
**namespace_awareness** | Option<**String**> |  | [optional]
**matches_x_path** | [**models::MatchesXpathPatternMatchesXPath**](matches_xpath_pattern_matchesXPath.md) |  | 
**x_path_namespaces** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**matches_json_schema** | **String** | A valid JSON schema as a string | 
**schema_version** | Option<**String**> | The JSON schema version to interpret the schema against | [optional]
**absent** | **bool** |  | 
**and** | [**Vec<models::ContentPattern>**](content-pattern.md) |  | 
**or** | [**Vec<models::ContentPattern>**](content-pattern.md) |  | 
**has_exactly** | [**Vec<models::ContentPattern>**](content-pattern.md) |  | 
**includes** | [**Vec<models::ContentPattern>**](content-pattern.md) |  | 
**expression** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


