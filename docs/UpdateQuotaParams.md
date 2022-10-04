# UpdateQuotaParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**String**> | Quota size (E.g. ``10000MB``, ``99 gb``, ``0.2T``, ``unlimited``) | [optional]
**default** | Option<[**crate::models::DefaultQuotaValues**](DefaultQuotaValues.md)> | Whether or not this is a default quota. Valid values are ``no``, ``unregistered``, ``registered``. Calling this method with ``default=\"no\"`` on a non-default quota will throw an error. Not passing this parameter is equivalent to passing ``no``. | [optional]
**description** | Option<**String**> | Detailed text description for this Quota. | [optional]
**in_groups** | Option<**Vec<String>**> | A list of group IDs or names to associate with this quota. | [optional]
**in_users** | Option<**Vec<String>**> | A list of user IDs or user emails to associate with this quota. | [optional]
**name** | Option<**String**> | The new name of the quota. This must be unique within a Galaxy instance. | [optional]
**operation** | Option<[**crate::models::QuotaOperation**](QuotaOperation.md)> | One of (``+``, ``-``, ``=``). If you wish to change this value, you must also provide the ``amount``, otherwise it will not take effect. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


