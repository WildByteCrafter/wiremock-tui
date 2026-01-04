# RequestPatternMultipartPatternsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**file_name** | Option<**String**> |  | [optional]
**matching_type** | Option<**String**> | Determines whether all or any of the parts must match the criteria for an overall match. | [optional][default to Any]
**headers** | Option<[**std::collections::HashMap<String, models::ContentPattern>**](content-pattern.md)> | Header patterns to match against in the <key>: { \"<predicate>\": \"<value>\" } form | [optional]
**body_patterns** | Option<[**Vec<models::ContentPattern>**](content-pattern.md)> | Body patterns to match against in the { \"<predicate>\": \"<value>\" } form | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


