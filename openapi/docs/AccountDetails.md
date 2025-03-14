# AccountDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **String** | Username. | 
**subscribed** | **bool** | Subscribed for the current season. | 
**status** | [**models::AccountStatus**](AccountStatus.md) | Member status. | 
**badges** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Account badges. | [optional]
**achievements_points** | **i32** | Achievement points. | 
**banned** | **bool** | Banned. | 
**ban_reason** | Option<**String**> | Ban reason. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


