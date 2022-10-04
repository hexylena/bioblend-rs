# CreateQuotaParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Quota size (E.g. ``10000MB``, ``99 gb``, ``0.2T``, ``unlimited``) | 
**default** | Option<[**crate::models::DefaultQuotaValues**](DefaultQuotaValues.md)> | Whether or not this is a default quota. Valid values are ``no``, ``unregistered``, ``registered``. None is equivalent to ``no``. | [optional]
**description** | **String** | Detailed text description for this Quota. | 
**in_groups** | Option<**Vec<String>**> | A list of group IDs or names to associate with this quota. | [optional][default to []]
**in_users** | Option<**Vec<String>**> | A list of user IDs or user emails to associate with this quota. | [optional][default to []]
**name** | **String** | The name of the quota. This must be unique within a Galaxy instance. | 
**operation** | Option<[**crate::models::QuotaOperation**](QuotaOperation.md)> | Quotas can have one of three `operations`:- `=` : The quota is exactly the amount specified- `+` : The amount specified will be added to the amounts of the user's other associated quota definitions- `-` : The amount specified will be subtracted from the amounts of the user's other associated quota definitions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


