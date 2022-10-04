# ResponseArchiveExportApiHistoriesIdExportsPut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**download_url** | **String** | Relative API URL to download the exported history archive. | 
**external_download_latest_url** | **String** | Fully qualified URL to download the latests version of the exported history archive. | 
**external_download_permanent_url** | **String** | Fully qualified URL to download this particular version of the exported history archive. | 
**id** | **String** | The encoded database ID of the job that is currently processing a particular request. | 
**job_id** | **String** | The encoded database ID of the job that is currently processing a particular request. | 
**preparing** | **bool** | Whether the history archive is currently being built or in preparation. | 
**ready** | **bool** | Whether the export history job has completed successfully and the archive is ready to download | 
**up_to_date** | **bool** | False, if a new export archive should be generated for the corresponding history. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


