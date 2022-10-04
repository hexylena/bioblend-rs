# DeleteHistoryContentPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purge** | Option<**bool**> | Whether to remove the dataset from storage. Datasets will only be removed from storage once all HDAs or LDDAs that refer to this datasets are deleted. | [optional][default to false]
**recursive** | Option<**bool**> | When deleting a dataset collection, whether to also delete containing datasets. | [optional][default to false]
**stop_job** | Option<**bool**> | Whether to stop the creating job if all the job's outputs are deleted. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


