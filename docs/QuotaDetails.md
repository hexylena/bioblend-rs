# QuotaDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bytes** | **i32** | The amount, expressed in bytes, of this Quota. | 
**default** | Option<[**Vec<crate::models::DefaultQuota>**](DefaultQuota.md)> | A list indicating which types of default user quotas, if any, are associated with this quota. | [optional][default to []]
**description** | **String** | Detailed text description for this Quota. | 
**display_amount** | **String** | Human-readable representation of the `amount` field. | 
**groups** | Option<[**Vec<crate::models::GroupQuota>**](GroupQuota.md)> | A list of specific groups of users associated with this quota. | [optional][default to []]
**id** | **String** | The `encoded identifier` of the quota. | 
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**name** | **String** | The name of the quota. This must be unique within a Galaxy instance. | 
**operation** | Option<[**crate::models::QuotaOperation**](QuotaOperation.md)> | Quotas can have one of three `operations`:- `=` : The quota is exactly the amount specified- `+` : The amount specified will be added to the amounts of the user's other associated quota definitions- `-` : The amount specified will be subtracted from the amounts of the user's other associated quota definitions | [optional]
**users** | Option<[**Vec<crate::models::UserQuota>**](UserQuota.md)> | A list of specific users associated with this quota. | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


