# Object

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hda_ldda** | Option<[**crate::models::DatasetSourceType**](DatasetSourceType.md)> | Whether this dataset belongs to a history (HDA) or a library (LDDA). | [optional]
**history_id** | **String** | The encoded ID of the history associated with this item. | 
**id** | **String** | The encoded ID of this entity. | 
**model_class** | Option<**String**> | The name of the database model class. | [optional]
**state** | [**crate::models::GalaxyModelDatasetStates**](galaxy__model__Dataset__states.md) | The current state of this dataset. | 
**accessible** | **bool** | Whether this item is accessible to the current user due to permissions. | 
**annotation** | **String** | An annotation to provide details or to help understand the purpose and usage of this item. | 
**api_type** | Option<**String**> | TODO | [optional]
**create_time** | Option<**String**> | The time and date this item was created. | [optional]
**created_from_basename** | Option<**String**> | The basename of the output that produced this dataset. | [optional]
**creating_job** | **String** | The encoded ID of the job that created this dataset. | 
**data_type** | **String** | The fully qualified name of the class implementing the data type of this dataset. | 
**dataset_id** | **String** | The encoded ID of the dataset associated with this item. | 
**deleted** | **bool** | Whether this item is marked as deleted. | 
**display_apps** | [**Vec<crate::models::DisplayApp>**](DisplayApp.md) | Contains new-style display app urls. | 
**display_types** | [**Vec<crate::models::DisplayApp>**](DisplayApp.md) | Contains old-style display app urls. | 
**download_url** | **String** | The URL to download this item from the server. | 
**extension** | **String** | The extension of the dataset. | 
**file_ext** | **String** | The extension of the file. | 
**file_name** | Option<**String**> | The full path to the dataset file. | [optional]
**file_size** | **i32** | The file size in bytes. | 
**genome_build** | Option<**String**> | TODO | [optional][default to ?]
**hid** | **i32** | The index position of this item in the History. | 
**history_content_type** | [**crate::models::HistoryContentType**](HistoryContentType.md) | The type of this item. | 
**meta_files** | [**Vec<crate::models::MetadataFile>**](MetadataFile.md) | Collection of metadata files associated with this dataset. | 
**metadata** | Option<[**serde_json::Value**](.md)> | The metadata associated with this dataset. | [optional]
**metadata_data_lines** | Option<**i32**> | TODO | [optional][default to 0]
**metadata_dbkey** | Option<**String**> | TODO | [optional][default to ?]
**misc_blurb** | Option<**String**> | TODO | [optional]
**misc_info** | Option<**String**> | TODO | [optional]
**name** | Option<**String**> | The name of the item. | [optional]
**peek** | Option<**String**> | A few lines of contents from the start of the file. | [optional]
**permissions** | [**crate::models::Permissions**](Permissions.md) |  | 
**purged** | **bool** | Whether this dataset has been removed from disk. | 
**rerunnable** | **bool** | Whether the job creating this dataset can be run again. | 
**resubmitted** | **bool** | Whether the job creating this dataset has been resubmitted. | 
**tags** | **Vec<String>** | The collection of tags associated with an item. | 
**r#type** | Option<**String**> | This is always `file` for datasets. | [optional]
**type_id** | Option<**String**> | The type and the encoded ID of this item. Used for caching. | [optional]
**update_time** | Option<**String**> | The last time and date this item was updated. | [optional]
**url** | **String** | The relative URL to access this item. | 
**uuid** | **String** | Universal unique identifier for this dataset. | 
**validated_state** | [**crate::models::ValidatedStates**](validated_states.md) | The state of the datatype validation for this dataset. | 
**validated_state_message** | **String** | The message with details about the datatype validation result for this dataset. | 
**visible** | **bool** | Whether this item is visible or hidden to the user by default. | 
**visualizations** | [**Vec<serde_json::Value>**](serde_json::Value.md) | The collection of visualizations that can be applied to this dataset. | 
**collection_type** | Option<**String**> | The type of the collection, can be `list`, `paired`, or define subcollections using `:` as separator like `list:paired` or `list:list`. | [optional]
**contents_url** | Option<**String**> | The relative URL to access the contents of this History. | [optional]
**element_count** | Option<**i32**> | The number of elements contained in the dataset collection. It may be None or undefined if the collection could not be populated. | [optional]
**elements** | Option<[**Vec<crate::models::DceSummary>**](DCESummary.md)> | The summary information of each of the elements inside the dataset collection. | [optional][default to []]
**populated** | Option<**bool**> | Whether the dataset collection elements (and any subcollections elements) were successfully populated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


