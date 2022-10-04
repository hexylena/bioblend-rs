# \HistoriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_api_histories_history_id_contents_archive_filename_format_get**](HistoriesApi.md#archive_api_histories_history_id_contents_archive_filename_format_get) | **GET** /api/histories/{history_id}/contents/archive/{filename}.{format} | Build and return a compressed archive of the selected history contents.
[**archive_api_histories_history_id_contents_archive_filename_format_get_0**](HistoriesApi.md#archive_api_histories_history_id_contents_archive_filename_format_get_0) | **GET** /api/histories/{history_id}/contents/archive/{filename}.{format} | Build and return a compressed archive of the selected history contents.
[**archive_api_histories_history_id_contents_archive_id_get**](HistoriesApi.md#archive_api_histories_history_id_contents_archive_id_get) | **GET** /api/histories/{history_id}/contents/archive/{id} | Build and return a compressed archive of the selected history contents.
[**archive_api_histories_history_id_contents_archive_id_get_0**](HistoriesApi.md#archive_api_histories_history_id_contents_archive_id_get_0) | **GET** /api/histories/{history_id}/contents/archive/{id} | Build and return a compressed archive of the selected history contents.
[**archive_export_api_histories_id_exports_put**](HistoriesApi.md#archive_export_api_histories_id_exports_put) | **PUT** /api/histories/{id}/exports | Start job (if needed) to create history export for corresponding history.
[**archive_export_api_histories_id_exports_put_0**](HistoriesApi.md#archive_export_api_histories_id_exports_put_0) | **PUT** /api/histories/{id}/exports | Start job (if needed) to create history export for corresponding history.
[**bulk_operation_api_histories_history_id_contents_bulk_put**](HistoriesApi.md#bulk_operation_api_histories_history_id_contents_bulk_put) | **PUT** /api/histories/{history_id}/contents/bulk | Executes an operation on a set of items contained in the given History.
[**bulk_operation_api_histories_history_id_contents_bulk_put_0**](HistoriesApi.md#bulk_operation_api_histories_history_id_contents_bulk_put_0) | **PUT** /api/histories/{history_id}/contents/bulk | Executes an operation on a set of items contained in the given History.
[**citations_api_histories_id_citations_get**](HistoriesApi.md#citations_api_histories_id_citations_get) | **GET** /api/histories/{id}/citations | Return all the citations for the tools used to produce the datasets in the history.
[**citations_api_histories_id_citations_get_0**](HistoriesApi.md#citations_api_histories_id_citations_get_0) | **GET** /api/histories/{id}/citations | Return all the citations for the tools used to produce the datasets in the history.
[**contents_near_api_histories_history_id_contents_direction_hid_limit_get**](HistoriesApi.md#contents_near_api_histories_history_id_contents_direction_hid_limit_get) | **GET** /api/histories/{history_id}/contents/{direction}/{hid}/{limit} | Get content items around a particular `HID`.
[**contents_near_api_histories_history_id_contents_direction_hid_limit_get_0**](HistoriesApi.md#contents_near_api_histories_history_id_contents_direction_hid_limit_get_0) | **GET** /api/histories/{history_id}/contents/{direction}/{hid}/{limit} | Get content items around a particular `HID`.
[**create_api_histories_history_id_contents_post**](HistoriesApi.md#create_api_histories_history_id_contents_post) | **POST** /api/histories/{history_id}/contents | Create a new `HDA` or `HDCA` in the given History.
[**create_api_histories_history_id_contents_post_0**](HistoriesApi.md#create_api_histories_history_id_contents_post_0) | **POST** /api/histories/{history_id}/contents | Create a new `HDA` or `HDCA` in the given History.
[**create_api_histories_history_id_contents_type_s_post**](HistoriesApi.md#create_api_histories_history_id_contents_type_s_post) | **POST** /api/histories/{history_id}/contents/{type}s | Create a new `HDA` or `HDCA` in the given History.
[**create_api_histories_history_id_contents_type_s_post_0**](HistoriesApi.md#create_api_histories_history_id_contents_type_s_post_0) | **POST** /api/histories/{history_id}/contents/{type}s | Create a new `HDA` or `HDCA` in the given History.
[**create_api_histories_post**](HistoriesApi.md#create_api_histories_post) | **POST** /api/histories | Creates a new history.
[**create_api_histories_post_0**](HistoriesApi.md#create_api_histories_post_0) | **POST** /api/histories | Creates a new history.
[**create_from_store_api_histories_from_store_post**](HistoriesApi.md#create_from_store_api_histories_from_store_post) | **POST** /api/histories/from_store | Create histories from a model store.
[**create_from_store_api_histories_from_store_post_0**](HistoriesApi.md#create_from_store_api_histories_from_store_post_0) | **POST** /api/histories/from_store | Create histories from a model store.
[**create_from_store_api_histories_history_id_contents_from_store_post**](HistoriesApi.md#create_from_store_api_histories_history_id_contents_from_store_post) | **POST** /api/histories/{history_id}/contents_from_store | Create contents from store.
[**create_from_store_api_histories_history_id_contents_from_store_post_0**](HistoriesApi.md#create_from_store_api_histories_history_id_contents_from_store_post_0) | **POST** /api/histories/{history_id}/contents_from_store | Create contents from store.
[**create_from_store_async_api_histories_from_store_async_post**](HistoriesApi.md#create_from_store_async_api_histories_from_store_async_post) | **POST** /api/histories/from_store_async | Launch a task to create histories from a model store.
[**create_from_store_async_api_histories_from_store_async_post_0**](HistoriesApi.md#create_from_store_async_api_histories_from_store_async_post_0) | **POST** /api/histories/from_store_async | Launch a task to create histories from a model store.
[**delete_api_histories_history_id_contents_id_delete**](HistoriesApi.md#delete_api_histories_history_id_contents_id_delete) | **DELETE** /api/histories/{history_id}/contents/{id} | Delete the history dataset with the given ``ID``.
[**delete_api_histories_history_id_contents_id_delete_0**](HistoriesApi.md#delete_api_histories_history_id_contents_id_delete_0) | **DELETE** /api/histories/{history_id}/contents/{id} | Delete the history dataset with the given ``ID``.
[**delete_api_histories_history_id_contents_type_sid_delete**](HistoriesApi.md#delete_api_histories_history_id_contents_type_sid_delete) | **DELETE** /api/histories/{history_id}/contents/{type}s/{id} | Delete the history content with the given ``ID`` and specified type.
[**delete_api_histories_history_id_contents_type_sid_delete_0**](HistoriesApi.md#delete_api_histories_history_id_contents_type_sid_delete_0) | **DELETE** /api/histories/{history_id}/contents/{type}s/{id} | Delete the history content with the given ``ID`` and specified type.
[**delete_api_histories_id_delete**](HistoriesApi.md#delete_api_histories_id_delete) | **DELETE** /api/histories/{id} | Marks the history with the given ID as deleted.
[**delete_api_histories_id_delete_0**](HistoriesApi.md#delete_api_histories_id_delete_0) | **DELETE** /api/histories/{id} | Marks the history with the given ID as deleted.
[**disable_link_access_api_histories_id_disable_link_access_put**](HistoriesApi.md#disable_link_access_api_histories_id_disable_link_access_put) | **PUT** /api/histories/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**disable_link_access_api_histories_id_disable_link_access_put_0**](HistoriesApi.md#disable_link_access_api_histories_id_disable_link_access_put_0) | **PUT** /api/histories/{id}/disable_link_access | Makes this item inaccessible by a URL link.
[**download_dataset_collection_api_dataset_collections_id_download_get**](HistoriesApi.md#download_dataset_collection_api_dataset_collections_id_download_get) | **GET** /api/dataset_collections/{id}/download | Download the content of a dataset collection as a `zip` archive.
[**download_dataset_collection_api_dataset_collections_id_download_get_0**](HistoriesApi.md#download_dataset_collection_api_dataset_collections_id_download_get_0) | **GET** /api/dataset_collections/{id}/download | Download the content of a dataset collection as a `zip` archive.
[**download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get**](HistoriesApi.md#download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get) | **GET** /api/histories/{history_id}/contents/dataset_collections/{id}/download | Download the content of a dataset collection as a `zip` archive.
[**download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get_0**](HistoriesApi.md#download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get_0) | **GET** /api/histories/{history_id}/contents/dataset_collections/{id}/download | Download the content of a dataset collection as a `zip` archive.
[**enable_link_access_api_histories_id_enable_link_access_put**](HistoriesApi.md#enable_link_access_api_histories_id_enable_link_access_put) | **PUT** /api/histories/{id}/enable_link_access | Makes this item accessible by a URL link.
[**enable_link_access_api_histories_id_enable_link_access_put_0**](HistoriesApi.md#enable_link_access_api_histories_id_enable_link_access_put_0) | **PUT** /api/histories/{id}/enable_link_access | Makes this item accessible by a URL link.
[**extra_files_api_histories_history_id_contents_history_content_id_extra_files_get**](HistoriesApi.md#extra_files_api_histories_history_id_contents_history_content_id_extra_files_get) | **GET** /api/histories/{history_id}/contents/{history_content_id}/extra_files | Generate list of extra files.
[**get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get**](HistoriesApi.md#get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get) | **GET** /api/histories/{id}/custom_builds_metadata | Returns meta data for custom builds.
[**get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get_0**](HistoriesApi.md#get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get_0) | **GET** /api/histories/{id}/custom_builds_metadata | Returns meta data for custom builds.
[**get_metadata_file_api_histories_history_id_contents_history_content_id_metadata_file_get**](HistoriesApi.md#get_metadata_file_api_histories_history_id_contents_history_content_id_metadata_file_get) | **GET** /api/histories/{history_id}/contents/{history_content_id}/metadata_file | Returns the metadata file associated with this history item.
[**history_api_histories_id_get**](HistoriesApi.md#history_api_histories_id_get) | **GET** /api/histories/{id} | Returns the history with the given ID.
[**history_api_histories_id_get_0**](HistoriesApi.md#history_api_histories_id_get_0) | **GET** /api/histories/{id} | Returns the history with the given ID.
[**history_archive_download_api_histories_id_exports_jeha_id_get**](HistoriesApi.md#history_archive_download_api_histories_id_exports_jeha_id_get) | **GET** /api/histories/{id}/exports/{jeha_id} | If ready and available, return raw contents of exported history as a downloadable archive.
[**history_archive_download_api_histories_id_exports_jeha_id_get_0**](HistoriesApi.md#history_archive_download_api_histories_id_exports_jeha_id_get_0) | **GET** /api/histories/{id}/exports/{jeha_id} | If ready and available, return raw contents of exported history as a downloadable archive.
[**history_content_api_histories_history_id_contents_id_get**](HistoriesApi.md#history_content_api_histories_history_id_contents_id_get) | **GET** /api/histories/{history_id}/contents/{id} | Return detailed information about an HDA within a history.
[**history_content_api_histories_history_id_contents_id_get_0**](HistoriesApi.md#history_content_api_histories_history_id_contents_id_get_0) | **GET** /api/histories/{history_id}/contents/{id} | Return detailed information about an HDA within a history.
[**history_content_typed_api_histories_history_id_contents_type_sid_get**](HistoriesApi.md#history_content_typed_api_histories_history_id_contents_type_sid_get) | **GET** /api/histories/{history_id}/contents/{type}s/{id} | Return detailed information about a specific HDA or HDCA with the given `ID` within a history.
[**history_content_typed_api_histories_history_id_contents_type_sid_get_0**](HistoriesApi.md#history_content_typed_api_histories_history_id_contents_type_sid_get_0) | **GET** /api/histories/{history_id}/contents/{type}s/{id} | Return detailed information about a specific HDA or HDCA with the given `ID` within a history.
[**history_contents_api_histories_history_id_contents_get**](HistoriesApi.md#history_contents_api_histories_history_id_contents_get) | **GET** /api/histories/{history_id}/contents | Returns the contents of the given history.
[**history_contents_api_histories_history_id_contents_get_0**](HistoriesApi.md#history_contents_api_histories_history_id_contents_get_0) | **GET** /api/histories/{history_id}/contents | Returns the contents of the given history.
[**history_contents_display_api_histories_history_id_contents_history_content_id_display_get**](HistoriesApi.md#history_contents_display_api_histories_history_id_contents_history_content_id_display_get) | **GET** /api/histories/{history_id}/contents/{history_content_id}/display | Displays (preview) or downloads dataset content.
[**index_api_histories_get**](HistoriesApi.md#index_api_histories_get) | **GET** /api/histories | Returns histories for the current user.
[**index_api_histories_get_0**](HistoriesApi.md#index_api_histories_get_0) | **GET** /api/histories | Returns histories for the current user.
[**index_api_histories_history_id_contents_type_s_get**](HistoriesApi.md#index_api_histories_history_id_contents_type_s_get) | **GET** /api/histories/{history_id}/contents/{type}s | Returns the contents of the given history filtered by type.
[**index_api_histories_history_id_contents_type_s_get_0**](HistoriesApi.md#index_api_histories_history_id_contents_type_s_get_0) | **GET** /api/histories/{history_id}/contents/{type}s | Returns the contents of the given history filtered by type.
[**index_deleted_api_histories_deleted_get**](HistoriesApi.md#index_deleted_api_histories_deleted_get) | **GET** /api/histories/deleted | Returns deleted histories for the current user.
[**index_deleted_api_histories_deleted_get_0**](HistoriesApi.md#index_deleted_api_histories_deleted_get_0) | **GET** /api/histories/deleted | Returns deleted histories for the current user.
[**index_exports_api_histories_id_exports_get**](HistoriesApi.md#index_exports_api_histories_id_exports_get) | **GET** /api/histories/{id}/exports | Get previous history exports (to links). Effectively returns serialized JEHA objects.
[**index_exports_api_histories_id_exports_get_0**](HistoriesApi.md#index_exports_api_histories_id_exports_get_0) | **GET** /api/histories/{id}/exports | Get previous history exports (to links). Effectively returns serialized JEHA objects.
[**index_jobs_summary_api_histories_history_id_jobs_summary_get**](HistoriesApi.md#index_jobs_summary_api_histories_history_id_jobs_summary_get) | **GET** /api/histories/{history_id}/jobs_summary | Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.
[**index_jobs_summary_api_histories_history_id_jobs_summary_get_0**](HistoriesApi.md#index_jobs_summary_api_histories_history_id_jobs_summary_get_0) | **GET** /api/histories/{history_id}/jobs_summary | Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.
[**materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post**](HistoriesApi.md#materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post) | **POST** /api/histories/{history_id}/contents/datasets/{id}/materialize | Materialize a deferred dataset into real, usable dataset.
[**materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post_0**](HistoriesApi.md#materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post_0) | **POST** /api/histories/{history_id}/contents/datasets/{id}/materialize | Materialize a deferred dataset into real, usable dataset.
[**materialize_to_history_api_histories_history_id_materialize_post**](HistoriesApi.md#materialize_to_history_api_histories_history_id_materialize_post) | **POST** /api/histories/{history_id}/materialize | Materialize a deferred library or HDA dataset into real, usable dataset in specified history.
[**materialize_to_history_api_histories_history_id_materialize_post_0**](HistoriesApi.md#materialize_to_history_api_histories_history_id_materialize_post_0) | **POST** /api/histories/{history_id}/materialize | Materialize a deferred library or HDA dataset into real, usable dataset in specified history.
[**prepare_collection_download_api_dataset_collections_id_prepare_download_post**](HistoriesApi.md#prepare_collection_download_api_dataset_collections_id_prepare_download_post) | **POST** /api/dataset_collections/{id}/prepare_download | Prepare an short term storage object that the collection will be downloaded to.
[**prepare_collection_download_api_dataset_collections_id_prepare_download_post_0**](HistoriesApi.md#prepare_collection_download_api_dataset_collections_id_prepare_download_post_0) | **POST** /api/dataset_collections/{id}/prepare_download | Prepare an short term storage object that the collection will be downloaded to.
[**prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post**](HistoriesApi.md#prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post) | **POST** /api/histories/{history_id}/contents/dataset_collections/{id}/prepare_download | Prepare an short term storage object that the collection will be downloaded to.
[**prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post_0**](HistoriesApi.md#prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post_0) | **POST** /api/histories/{history_id}/contents/dataset_collections/{id}/prepare_download | Prepare an short term storage object that the collection will be downloaded to.
[**prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post**](HistoriesApi.md#prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post) | **POST** /api/histories/{history_id}/contents/{type}s/{id}/prepare_store_download | Prepare a dataset or dataset collection for export-style download.
[**prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post_0**](HistoriesApi.md#prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post_0) | **POST** /api/histories/{history_id}/contents/{type}s/{id}/prepare_store_download | Prepare a dataset or dataset collection for export-style download.
[**prepare_store_download_api_histories_id_prepare_store_download_post**](HistoriesApi.md#prepare_store_download_api_histories_id_prepare_store_download_post) | **POST** /api/histories/{id}/prepare_store_download | Return a short term storage token to monitor download of the history.
[**prepare_store_download_api_histories_id_prepare_store_download_post_0**](HistoriesApi.md#prepare_store_download_api_histories_id_prepare_store_download_post_0) | **POST** /api/histories/{id}/prepare_store_download | Return a short term storage token to monitor download of the history.
[**publish_api_histories_id_publish_put**](HistoriesApi.md#publish_api_histories_id_publish_put) | **PUT** /api/histories/{id}/publish | Makes this item public and accessible by a URL link.
[**publish_api_histories_id_publish_put_0**](HistoriesApi.md#publish_api_histories_id_publish_put_0) | **PUT** /api/histories/{id}/publish | Makes this item public and accessible by a URL link.
[**published_api_histories_published_get**](HistoriesApi.md#published_api_histories_published_get) | **GET** /api/histories/published | Return all histories that are published.
[**published_api_histories_published_get_0**](HistoriesApi.md#published_api_histories_published_get_0) | **GET** /api/histories/published | Return all histories that are published.
[**set_slug_api_histories_id_slug_put**](HistoriesApi.md#set_slug_api_histories_id_slug_put) | **PUT** /api/histories/{id}/slug | Set a new slug for this shared item.
[**set_slug_api_histories_id_slug_put_0**](HistoriesApi.md#set_slug_api_histories_id_slug_put_0) | **PUT** /api/histories/{id}/slug | Set a new slug for this shared item.
[**share_with_users_api_histories_id_share_with_users_put**](HistoriesApi.md#share_with_users_api_histories_id_share_with_users_put) | **PUT** /api/histories/{id}/share_with_users | Share this item with specific users.
[**share_with_users_api_histories_id_share_with_users_put_0**](HistoriesApi.md#share_with_users_api_histories_id_share_with_users_put_0) | **PUT** /api/histories/{id}/share_with_users | Share this item with specific users.
[**shared_with_me_api_histories_shared_with_me_get**](HistoriesApi.md#shared_with_me_api_histories_shared_with_me_get) | **GET** /api/histories/shared_with_me | Return all histories that are shared with the current user.
[**shared_with_me_api_histories_shared_with_me_get_0**](HistoriesApi.md#shared_with_me_api_histories_shared_with_me_get_0) | **GET** /api/histories/shared_with_me | Return all histories that are shared with the current user.
[**sharing_api_histories_id_sharing_get**](HistoriesApi.md#sharing_api_histories_id_sharing_get) | **GET** /api/histories/{id}/sharing | Get the current sharing status of the given item.
[**sharing_api_histories_id_sharing_get_0**](HistoriesApi.md#sharing_api_histories_id_sharing_get_0) | **GET** /api/histories/{id}/sharing | Get the current sharing status of the given item.
[**show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get**](HistoriesApi.md#show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get) | **GET** /api/histories/{history_id}/contents/{type}s/{id}/jobs_summary | Return detailed information about an `HDA` or `HDCAs` jobs.
[**show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get_0**](HistoriesApi.md#show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get_0) | **GET** /api/histories/{history_id}/contents/{type}s/{id}/jobs_summary | Return detailed information about an `HDA` or `HDCAs` jobs.
[**show_recent_api_histories_most_recently_used_get**](HistoriesApi.md#show_recent_api_histories_most_recently_used_get) | **GET** /api/histories/most_recently_used | Returns the most recently used history of the user.
[**show_recent_api_histories_most_recently_used_get_0**](HistoriesApi.md#show_recent_api_histories_most_recently_used_get_0) | **GET** /api/histories/most_recently_used | Returns the most recently used history of the user.
[**undelete_api_histories_deleted_id_undelete_post**](HistoriesApi.md#undelete_api_histories_deleted_id_undelete_post) | **POST** /api/histories/deleted/{id}/undelete | Restores a deleted history with the given ID (that hasn't been purged).
[**undelete_api_histories_deleted_id_undelete_post_0**](HistoriesApi.md#undelete_api_histories_deleted_id_undelete_post_0) | **POST** /api/histories/deleted/{id}/undelete | Restores a deleted history with the given ID (that hasn't been purged).
[**unpublish_api_histories_id_unpublish_put**](HistoriesApi.md#unpublish_api_histories_id_unpublish_put) | **PUT** /api/histories/{id}/unpublish | Removes this item from the published list.
[**unpublish_api_histories_id_unpublish_put_0**](HistoriesApi.md#unpublish_api_histories_id_unpublish_put_0) | **PUT** /api/histories/{id}/unpublish | Removes this item from the published list.
[**update_api_histories_history_id_contents_id_put**](HistoriesApi.md#update_api_histories_history_id_contents_id_put) | **PUT** /api/histories/{history_id}/contents/{id} | Updates the values for the history content item with the given ``ID``.
[**update_api_histories_history_id_contents_id_put_0**](HistoriesApi.md#update_api_histories_history_id_contents_id_put_0) | **PUT** /api/histories/{history_id}/contents/{id} | Updates the values for the history content item with the given ``ID``.
[**update_api_histories_history_id_contents_type_sid_put**](HistoriesApi.md#update_api_histories_history_id_contents_type_sid_put) | **PUT** /api/histories/{history_id}/contents/{type}s/{id} | Updates the values for the history content item with the given ``ID``.
[**update_api_histories_history_id_contents_type_sid_put_0**](HistoriesApi.md#update_api_histories_history_id_contents_type_sid_put_0) | **PUT** /api/histories/{history_id}/contents/{type}s/{id} | Updates the values for the history content item with the given ``ID``.
[**update_api_histories_id_put**](HistoriesApi.md#update_api_histories_id_put) | **PUT** /api/histories/{id} | Updates the values for the history with the given ID.
[**update_api_histories_id_put_0**](HistoriesApi.md#update_api_histories_id_put_0) | **PUT** /api/histories/{id} | Updates the values for the history with the given ID.
[**update_batch_api_histories_history_id_contents_put**](HistoriesApi.md#update_batch_api_histories_history_id_contents_put) | **PUT** /api/histories/{history_id}/contents | Batch update specific properties of a set items contained in the given History.
[**update_batch_api_histories_history_id_contents_put_0**](HistoriesApi.md#update_batch_api_histories_history_id_contents_put_0) | **PUT** /api/histories/{history_id}/contents | Batch update specific properties of a set items contained in the given History.
[**update_permissions_api_histories_history_id_contents_dataset_id_permissions_put**](HistoriesApi.md#update_permissions_api_histories_history_id_contents_dataset_id_permissions_put) | **PUT** /api/histories/{history_id}/contents/{dataset_id}/permissions | Set permissions of the given history dataset to the given role ids.
[**update_permissions_api_histories_history_id_contents_dataset_id_permissions_put_0**](HistoriesApi.md#update_permissions_api_histories_history_id_contents_dataset_id_permissions_put_0) | **PUT** /api/histories/{history_id}/contents/{dataset_id}/permissions | Set permissions of the given history dataset to the given role ids.
[**validate_api_histories_history_id_contents_id_validate_put**](HistoriesApi.md#validate_api_histories_history_id_contents_id_validate_put) | **PUT** /api/histories/{history_id}/contents/{id}/validate | Validates the metadata associated with a dataset within a History.
[**validate_api_histories_history_id_contents_id_validate_put_0**](HistoriesApi.md#validate_api_histories_history_id_contents_id_validate_put_0) | **PUT** /api/histories/{history_id}/contents/{id}/validate | Validates the metadata associated with a dataset within a History.
[**write_store_api_histories_history_id_contents_type_sid_write_store_post**](HistoriesApi.md#write_store_api_histories_history_id_contents_type_sid_write_store_post) | **POST** /api/histories/{history_id}/contents/{type}s/{id}/write_store | Prepare a dataset or dataset collection for export-style download and write to supplied URI.
[**write_store_api_histories_history_id_contents_type_sid_write_store_post_0**](HistoriesApi.md#write_store_api_histories_history_id_contents_type_sid_write_store_post_0) | **POST** /api/histories/{history_id}/contents/{type}s/{id}/write_store | Prepare a dataset or dataset collection for export-style download and write to supplied URI.
[**write_store_api_histories_id_write_store_post**](HistoriesApi.md#write_store_api_histories_id_write_store_post) | **POST** /api/histories/{id}/write_store | Prepare history for export-style download and write to supplied URI.
[**write_store_api_histories_id_write_store_post_0**](HistoriesApi.md#write_store_api_histories_id_write_store_post_0) | **POST** /api/histories/{id}/write_store | Prepare history for export-style download and write to supplied URI.



## archive_api_histories_history_id_contents_archive_filename_format_get

> serde_json::Value archive_api_histories_history_id_contents_archive_filename_format_get(history_id, filename, format, dry_run, q, qv, offset, limit, order, run_as)
Build and return a compressed archive of the selected history contents.

Build and return a compressed archive of the selected history contents.  **Note**: this is a volatile endpoint and settings and behavior may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**filename** | **String** |  | [required] |
**format** | **String** |  | [required] |
**dry_run** | Option<**bool**> | Whether to return the archive and file paths only (as JSON) and not an actual archive file. |  |[default to true]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_api_histories_history_id_contents_archive_filename_format_get_0

> serde_json::Value archive_api_histories_history_id_contents_archive_filename_format_get_0(history_id, filename, format, dry_run, q, qv, offset, limit, order, run_as)
Build and return a compressed archive of the selected history contents.

Build and return a compressed archive of the selected history contents.  **Note**: this is a volatile endpoint and settings and behavior may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**filename** | **String** |  | [required] |
**format** | **String** |  | [required] |
**dry_run** | Option<**bool**> | Whether to return the archive and file paths only (as JSON) and not an actual archive file. |  |[default to true]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_api_histories_history_id_contents_archive_id_get

> serde_json::Value archive_api_histories_history_id_contents_archive_id_get(history_id, filename, format, dry_run, q, qv, offset, limit, order, run_as)
Build and return a compressed archive of the selected history contents.

Build and return a compressed archive of the selected history contents.  **Note**: this is a volatile endpoint and settings and behavior may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**filename** | Option<**String**> | The name that the Archive will have (defaults to history name). |  |
**format** | Option<**String**> | Output format of the archive. |  |[default to zip]
**dry_run** | Option<**bool**> | Whether to return the archive and file paths only (as JSON) and not an actual archive file. |  |[default to true]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_api_histories_history_id_contents_archive_id_get_0

> serde_json::Value archive_api_histories_history_id_contents_archive_id_get_0(history_id, filename, format, dry_run, q, qv, offset, limit, order, run_as)
Build and return a compressed archive of the selected history contents.

Build and return a compressed archive of the selected history contents.  **Note**: this is a volatile endpoint and settings and behavior may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**filename** | Option<**String**> | The name that the Archive will have (defaults to history name). |  |
**format** | Option<**String**> | Output format of the archive. |  |[default to zip]
**dry_run** | Option<**bool**> | Whether to return the archive and file paths only (as JSON) and not an actual archive file. |  |[default to true]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_export_api_histories_id_exports_put

> crate::models::ResponseArchiveExportApiHistoriesIdExportsPut archive_export_api_histories_id_exports_put(id, run_as, export_history_archive_payload)
Start job (if needed) to create history export for corresponding history.

This will start a job to create a history export archive.  Calling this endpoint multiple times will return the 202 status code until the archive has been completely generated and is ready to download. When ready, it will return the 200 status code along with the download link information.  If the history will be exported to a `directory_uri`, instead of returning the download link information, the Job ID will be returned so it can be queried to determine when the file has been written.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**export_history_archive_payload** | Option<[**ExportHistoryArchivePayload**](ExportHistoryArchivePayload.md)> |  |  |

### Return type

[**crate::models::ResponseArchiveExportApiHistoriesIdExportsPut**](Response_Archive_Export_Api_Histories__Id__Exports_Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_export_api_histories_id_exports_put_0

> crate::models::ResponseArchiveExportApiHistoriesIdExportsPut archive_export_api_histories_id_exports_put_0(id, run_as, export_history_archive_payload)
Start job (if needed) to create history export for corresponding history.

This will start a job to create a history export archive.  Calling this endpoint multiple times will return the 202 status code until the archive has been completely generated and is ready to download. When ready, it will return the 200 status code along with the download link information.  If the history will be exported to a `directory_uri`, instead of returning the download link information, the Job ID will be returned so it can be queried to determine when the file has been written.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**export_history_archive_payload** | Option<[**ExportHistoryArchivePayload**](ExportHistoryArchivePayload.md)> |  |  |

### Return type

[**crate::models::ResponseArchiveExportApiHistoriesIdExportsPut**](Response_Archive_Export_Api_Histories__Id__Exports_Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_operation_api_histories_history_id_contents_bulk_put

> crate::models::HistoryContentBulkOperationResult bulk_operation_api_histories_history_id_contents_bulk_put(history_id, history_content_bulk_operation_payload, q, qv, run_as)
Executes an operation on a set of items contained in the given History.

Executes an operation on a set of items contained in the given History.  The items to be processed can be explicitly set or determined by a dynamic query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**history_content_bulk_operation_payload** | [**HistoryContentBulkOperationPayload**](HistoryContentBulkOperationPayload.md) |  | [required] |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HistoryContentBulkOperationResult**](HistoryContentBulkOperationResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_operation_api_histories_history_id_contents_bulk_put_0

> crate::models::HistoryContentBulkOperationResult bulk_operation_api_histories_history_id_contents_bulk_put_0(history_id, history_content_bulk_operation_payload, q, qv, run_as)
Executes an operation on a set of items contained in the given History.

Executes an operation on a set of items contained in the given History.  The items to be processed can be explicitly set or determined by a dynamic query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**history_content_bulk_operation_payload** | [**HistoryContentBulkOperationPayload**](HistoryContentBulkOperationPayload.md) |  | [required] |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HistoryContentBulkOperationResult**](HistoryContentBulkOperationResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## citations_api_histories_id_citations_get

> Vec<serde_json::Value> citations_api_histories_id_citations_get(id, run_as)
Return all the citations for the tools used to produce the datasets in the history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## citations_api_histories_id_citations_get_0

> Vec<serde_json::Value> citations_api_histories_id_citations_get_0(id, run_as)
Return all the citations for the tools used to produce the datasets in the history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contents_near_api_histories_history_id_contents_direction_hid_limit_get

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> contents_near_api_histories_history_id_contents_direction_hid_limit_get(history_id, hid, direction, limit, since, view, keys, run_as)
Get content items around a particular `HID`.

.. warning:: For internal use to support the scroller functionality.  This endpoint provides random access to a large history without having to know exactly how many pages are in the final query. Pick a target HID and filters, and the endpoint will get a maximum of `limit` history items \"around\" the `hid`.  Additional counts are provided in the HTTP headers.  The `direction` determines what items are selected:  a) item counts:     - total matches-up:   hid < {hid}    - total matches-down: hid > {hid}    - total matches:      total matches-up + total matches-down + 1 (+1 for hid == {hid})    - displayed matches-up:   hid <= {hid} (hid == {hid} is included)    - displayed matches-down: hid > {hid}    - displayed matches:      displayed matches-up + displayed matches-down  b) {limit} history items:     - if direction == \"before\": hid <= {hid}    - if direction == \"after\":  hid > {hid}    - if direction == \"near\":   \"near\" {hid}, so that      n. items before <= limit // 2,      n. items after <= limit // 2 + 1.  .. note:: This endpoint uses slightly different filter params syntax. Instead of using `q`/`qv` parameters,     it uses the following syntax for query parameters::          ?[field]-[operator]=[value]      Example::          ?update_time-gt=2015-01-29

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**hid** | **i32** | The target `HID` to get content around it. | [required] |
**direction** | [**crate::models::DirectionOptions**](.md) | Determines what items are selected before, after or near the target `hid`. | [required] |
**limit** | **i32** | The maximum number of content items to return above and below the target `HID`. | [required] |
**since** | Option<**String**> | A timestamp in ISO format to check if the history has changed since this particular date/time. If it hasn't changed, no additional processing will be done and 204 status code will be returned. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contents_near_api_histories_history_id_contents_direction_hid_limit_get_0

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> contents_near_api_histories_history_id_contents_direction_hid_limit_get_0(history_id, hid, direction, limit, since, view, keys, run_as)
Get content items around a particular `HID`.

.. warning:: For internal use to support the scroller functionality.  This endpoint provides random access to a large history without having to know exactly how many pages are in the final query. Pick a target HID and filters, and the endpoint will get a maximum of `limit` history items \"around\" the `hid`.  Additional counts are provided in the HTTP headers.  The `direction` determines what items are selected:  a) item counts:     - total matches-up:   hid < {hid}    - total matches-down: hid > {hid}    - total matches:      total matches-up + total matches-down + 1 (+1 for hid == {hid})    - displayed matches-up:   hid <= {hid} (hid == {hid} is included)    - displayed matches-down: hid > {hid}    - displayed matches:      displayed matches-up + displayed matches-down  b) {limit} history items:     - if direction == \"before\": hid <= {hid}    - if direction == \"after\":  hid > {hid}    - if direction == \"near\":   \"near\" {hid}, so that      n. items before <= limit // 2,      n. items after <= limit // 2 + 1.  .. note:: This endpoint uses slightly different filter params syntax. Instead of using `q`/`qv` parameters,     it uses the following syntax for query parameters::          ?[field]-[operator]=[value]      Example::          ?update_time-gt=2015-01-29

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**hid** | **i32** | The target `HID` to get content around it. | [required] |
**direction** | [**crate::models::DirectionOptions**](.md) | Determines what items are selected before, after or near the target `hid`. | [required] |
**limit** | **i32** | The maximum number of content items to return above and below the target `HID`. | [required] |
**since** | Option<**String**> | A timestamp in ISO format to check if the history has changed since this particular date/time. If it hasn't changed, no additional processing will be done and 204 status code will be returned. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_history_id_contents_post

> crate::models::ResponseCreateApiHistoriesHistoryIdContentsPost create_api_histories_history_id_contents_post(history_id, create_history_content_payload, r#type, view, keys, run_as)
Create a new `HDA` or `HDCA` in the given History.

Create a new `HDA` or `HDCA` in the given History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**create_history_content_payload** | [**CreateHistoryContentPayload**](CreateHistoryContentPayload.md) |  | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to create. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesHistoryIdContentsPost**](Response_Create_Api_Histories__History_Id__Contents_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_history_id_contents_post_0

> crate::models::ResponseCreateApiHistoriesHistoryIdContentsPost create_api_histories_history_id_contents_post_0(history_id, create_history_content_payload, r#type, view, keys, run_as)
Create a new `HDA` or `HDCA` in the given History.

Create a new `HDA` or `HDCA` in the given History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**create_history_content_payload** | [**CreateHistoryContentPayload**](CreateHistoryContentPayload.md) |  | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to create. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesHistoryIdContentsPost**](Response_Create_Api_Histories__History_Id__Contents_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_history_id_contents_type_s_post

> crate::models::ResponseCreateApiHistoriesHistoryIdContentsTypeSPost create_api_histories_history_id_contents_type_s_post(history_id, r#type, create_history_content_payload, view, keys, run_as)
Create a new `HDA` or `HDCA` in the given History.

Create a new `HDA` or `HDCA` in the given History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**create_history_content_payload** | [**CreateHistoryContentPayload**](CreateHistoryContentPayload.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesHistoryIdContentsTypeSPost**](Response_Create_Api_Histories__History_Id__Contents__Type_S_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_history_id_contents_type_s_post_0

> crate::models::ResponseCreateApiHistoriesHistoryIdContentsTypeSPost create_api_histories_history_id_contents_type_s_post_0(history_id, r#type, create_history_content_payload, view, keys, run_as)
Create a new `HDA` or `HDCA` in the given History.

Create a new `HDA` or `HDCA` in the given History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**create_history_content_payload** | [**CreateHistoryContentPayload**](CreateHistoryContentPayload.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesHistoryIdContentsTypeSPost**](Response_Create_Api_Histories__History_Id__Contents__Type_S_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_post

> crate::models::ResponseCreateApiHistoriesPost create_api_histories_post(view, keys, run_as, all_datasets, archive_file, archive_source, archive_type, history_id, name)
Creates a new history.

The new history can also be copied form a existing history or imported from an archive or URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**all_datasets** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_file** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_source** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_type** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**history_id** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**name** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesPost**](Response_Create_Api_Histories_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_histories_post_0

> crate::models::ResponseCreateApiHistoriesPost create_api_histories_post_0(view, keys, run_as, all_datasets, archive_file, archive_source, archive_type, history_id, name)
Creates a new history.

The new history can also be copied form a existing history or imported from an archive or URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**all_datasets** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_file** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_source** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**archive_type** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**history_id** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**name** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::ResponseCreateApiHistoriesPost**](Response_Create_Api_Histories_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_histories_from_store_post

> crate::models::ResponseCreateFromStoreApiHistoriesFromStorePost create_from_store_api_histories_from_store_post(create_history_from_store, view, keys, run_as)
Create histories from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_history_from_store** | [**CreateHistoryFromStore**](CreateHistoryFromStore.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateFromStoreApiHistoriesFromStorePost**](Response_Create_From_Store_Api_Histories_From_Store_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_histories_from_store_post_0

> crate::models::ResponseCreateFromStoreApiHistoriesFromStorePost create_from_store_api_histories_from_store_post_0(create_history_from_store, view, keys, run_as)
Create histories from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_history_from_store** | [**CreateHistoryFromStore**](CreateHistoryFromStore.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateFromStoreApiHistoriesFromStorePost**](Response_Create_From_Store_Api_Histories_From_Store_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_histories_history_id_contents_from_store_post

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> create_from_store_api_histories_history_id_contents_from_store_post(history_id, create_history_content_from_store, view, keys, run_as)
Create contents from store.

Create history contents from model store. Input can be a tarfile created with build_objects script distributed with galaxy-data, from an exported history with files stripped out, or hand-crafted JSON dictionary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**create_history_content_from_store** | [**CreateHistoryContentFromStore**](CreateHistoryContentFromStore.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_api_histories_history_id_contents_from_store_post_0

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> create_from_store_api_histories_history_id_contents_from_store_post_0(history_id, create_history_content_from_store, view, keys, run_as)
Create contents from store.

Create history contents from model store. Input can be a tarfile created with build_objects script distributed with galaxy-data, from an exported history with files stripped out, or hand-crafted JSON dictionary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**create_history_content_from_store** | [**CreateHistoryContentFromStore**](CreateHistoryContentFromStore.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_async_api_histories_from_store_async_post

> crate::models::ResponseCreateFromStoreAsyncApiHistoriesFromStoreAsyncPost create_from_store_async_api_histories_from_store_async_post(create_history_from_store, run_as)
Launch a task to create histories from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_history_from_store** | [**CreateHistoryFromStore**](CreateHistoryFromStore.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateFromStoreAsyncApiHistoriesFromStoreAsyncPost**](Response_Create_From_Store_Async_Api_Histories_From_Store_Async_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_from_store_async_api_histories_from_store_async_post_0

> crate::models::ResponseCreateFromStoreAsyncApiHistoriesFromStoreAsyncPost create_from_store_async_api_histories_from_store_async_post_0(create_history_from_store, run_as)
Launch a task to create histories from a model store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_history_from_store** | [**CreateHistoryFromStore**](CreateHistoryFromStore.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseCreateFromStoreAsyncApiHistoriesFromStoreAsyncPost**](Response_Create_From_Store_Async_Api_Histories_From_Store_Async_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_history_id_contents_id_delete

> crate::models::DeleteHistoryContentResult delete_api_histories_history_id_contents_id_delete(history_id, id, r#type, purge, recursive, stop_job, view, keys, run_as, delete_history_content_payload)
Delete the history dataset with the given ``ID``.

Delete the history content with the given ``ID`` and specified type (defaults to dataset).  **Note**: Currently does not stop any active jobs for which this dataset is an output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**purge** | Option<**bool**> | Whether to remove from disk the target HDA or child HDAs of the target HDCA. |  |[default to false]
**recursive** | Option<**bool**> | When deleting a dataset collection, whether to also delete containing datasets. |  |[default to false]
**stop_job** | Option<**bool**> | Whether to stop the creating job if all outputs of the job have been deleted. |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_content_payload** | Option<[**DeleteHistoryContentPayload**](DeleteHistoryContentPayload.md)> |  |  |

### Return type

[**crate::models::DeleteHistoryContentResult**](DeleteHistoryContentResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_history_id_contents_id_delete_0

> crate::models::DeleteHistoryContentResult delete_api_histories_history_id_contents_id_delete_0(history_id, id, r#type, purge, recursive, stop_job, view, keys, run_as, delete_history_content_payload)
Delete the history dataset with the given ``ID``.

Delete the history content with the given ``ID`` and specified type (defaults to dataset).  **Note**: Currently does not stop any active jobs for which this dataset is an output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**purge** | Option<**bool**> | Whether to remove from disk the target HDA or child HDAs of the target HDCA. |  |[default to false]
**recursive** | Option<**bool**> | When deleting a dataset collection, whether to also delete containing datasets. |  |[default to false]
**stop_job** | Option<**bool**> | Whether to stop the creating job if all outputs of the job have been deleted. |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_content_payload** | Option<[**DeleteHistoryContentPayload**](DeleteHistoryContentPayload.md)> |  |  |

### Return type

[**crate::models::DeleteHistoryContentResult**](DeleteHistoryContentResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_history_id_contents_type_sid_delete

> crate::models::DeleteHistoryContentResult delete_api_histories_history_id_contents_type_sid_delete(history_id, id, r#type, purge, recursive, stop_job, view, keys, run_as, delete_history_content_payload)
Delete the history content with the given ``ID`` and specified type.

Delete the history content with the given ``ID`` and specified type (defaults to dataset).  **Note**: Currently does not stop any active jobs for which this dataset is an output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**purge** | Option<**bool**> | Whether to remove from disk the target HDA or child HDAs of the target HDCA. |  |[default to false]
**recursive** | Option<**bool**> | When deleting a dataset collection, whether to also delete containing datasets. |  |[default to false]
**stop_job** | Option<**bool**> | Whether to stop the creating job if all outputs of the job have been deleted. |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_content_payload** | Option<[**DeleteHistoryContentPayload**](DeleteHistoryContentPayload.md)> |  |  |

### Return type

[**crate::models::DeleteHistoryContentResult**](DeleteHistoryContentResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_history_id_contents_type_sid_delete_0

> crate::models::DeleteHistoryContentResult delete_api_histories_history_id_contents_type_sid_delete_0(history_id, id, r#type, purge, recursive, stop_job, view, keys, run_as, delete_history_content_payload)
Delete the history content with the given ``ID`` and specified type.

Delete the history content with the given ``ID`` and specified type (defaults to dataset).  **Note**: Currently does not stop any active jobs for which this dataset is an output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**purge** | Option<**bool**> | Whether to remove from disk the target HDA or child HDAs of the target HDCA. |  |[default to false]
**recursive** | Option<**bool**> | When deleting a dataset collection, whether to also delete containing datasets. |  |[default to false]
**stop_job** | Option<**bool**> | Whether to stop the creating job if all outputs of the job have been deleted. |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_content_payload** | Option<[**DeleteHistoryContentPayload**](DeleteHistoryContentPayload.md)> |  |  |

### Return type

[**crate::models::DeleteHistoryContentResult**](DeleteHistoryContentResult.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_id_delete

> crate::models::ResponseDeleteApiHistoriesIdDelete delete_api_histories_id_delete(id, purge, view, keys, run_as, delete_history_payload)
Marks the history with the given ID as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**purge** | Option<**bool**> |  |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_payload** | Option<[**DeleteHistoryPayload**](DeleteHistoryPayload.md)> |  |  |

### Return type

[**crate::models::ResponseDeleteApiHistoriesIdDelete**](Response_Delete_Api_Histories__Id__Delete.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_histories_id_delete_0

> crate::models::ResponseDeleteApiHistoriesIdDelete delete_api_histories_id_delete_0(id, purge, view, keys, run_as, delete_history_payload)
Marks the history with the given ID as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**purge** | Option<**bool**> |  |  |[default to false]
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |
**delete_history_payload** | Option<[**DeleteHistoryPayload**](DeleteHistoryPayload.md)> |  |  |

### Return type

[**crate::models::ResponseDeleteApiHistoriesIdDelete**](Response_Delete_Api_Histories__Id__Delete.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_histories_id_disable_link_access_put

> crate::models::SharingStatus disable_link_access_api_histories_id_disable_link_access_put(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_link_access_api_histories_id_disable_link_access_put_0

> crate::models::SharingStatus disable_link_access_api_histories_id_disable_link_access_put_0(id, run_as)
Makes this item inaccessible by a URL link.

Makes this item inaccessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_dataset_collection_api_dataset_collections_id_download_get

> download_dataset_collection_api_dataset_collections_id_download_get(id, history_id, run_as)
Download the content of a dataset collection as a `zip` archive.

Download the content of a history dataset collection as a `zip` archive while maintaining approximate collection structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**history_id** | Option<**String**> | The encoded database identifier of the History. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_dataset_collection_api_dataset_collections_id_download_get_0

> download_dataset_collection_api_dataset_collections_id_download_get_0(id, history_id, run_as)
Download the content of a dataset collection as a `zip` archive.

Download the content of a history dataset collection as a `zip` archive while maintaining approximate collection structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**history_id** | Option<**String**> | The encoded database identifier of the History. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get

> download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get(history_id, id, run_as)
Download the content of a dataset collection as a `zip` archive.

Download the content of a history dataset collection as a `zip` archive while maintaining approximate collection structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** |  | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get_0

> download_dataset_collection_api_histories_history_id_contents_dataset_collections_id_download_get_0(history_id, id, run_as)
Download the content of a dataset collection as a `zip` archive.

Download the content of a history dataset collection as a `zip` archive while maintaining approximate collection structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** |  | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_histories_id_enable_link_access_put

> crate::models::SharingStatus enable_link_access_api_histories_id_enable_link_access_put(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_link_access_api_histories_id_enable_link_access_put_0

> crate::models::SharingStatus enable_link_access_api_histories_id_enable_link_access_put_0(id, run_as)
Makes this item accessible by a URL link.

Makes this item accessible by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extra_files_api_histories_history_id_contents_history_content_id_extra_files_get

> serde_json::Value extra_files_api_histories_history_id_contents_history_content_id_extra_files_get(history_id, history_content_id, run_as)
Generate list of extra files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The encoded database identifier of the History. | [required] |
**history_content_id** | **String** | The encoded database identifier of the dataset. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get

> crate::models::CustomBuildsMetadataResponse get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get(id, run_as)
Returns meta data for custom builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::CustomBuildsMetadataResponse**](CustomBuildsMetadataResponse.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get_0

> crate::models::CustomBuildsMetadataResponse get_custom_builds_metadata_api_histories_id_custom_builds_metadata_get_0(id, run_as)
Returns meta data for custom builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::CustomBuildsMetadataResponse**](CustomBuildsMetadataResponse.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_file_api_histories_history_id_contents_history_content_id_metadata_file_get

> get_metadata_file_api_histories_history_id_contents_history_content_id_metadata_file_get(history_id, history_content_id, metadata_file, run_as)
Returns the metadata file associated with this history item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** |  | [required] |
**history_content_id** | **String** | The encoded database identifier of the dataset. | [required] |
**metadata_file** | **String** | The name of the metadata file to retrieve. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_api_histories_id_get

> crate::models::ResponseHistoryApiHistoriesIdGet history_api_histories_id_get(id, view, keys, run_as)
Returns the history with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryApiHistoriesIdGet**](Response_History_Api_Histories__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_api_histories_id_get_0

> crate::models::ResponseHistoryApiHistoriesIdGet history_api_histories_id_get_0(id, view, keys, run_as)
Returns the history with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryApiHistoriesIdGet**](Response_History_Api_Histories__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_archive_download_api_histories_id_exports_jeha_id_get

> history_archive_download_api_histories_id_exports_jeha_id_get(id, jeha_id, run_as)
If ready and available, return raw contents of exported history as a downloadable archive.

See ``PUT /api/histories/{id}/exports`` to initiate the creation of the history export - when ready, that route will return 200 status code (instead of 202) and this route can be used to download the archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**jeha_id** | [**JobExportHistoryId**](.md) | The ID of the specific Job Export History Association or `latest` (default) to download the last generated archive. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_archive_download_api_histories_id_exports_jeha_id_get_0

> history_archive_download_api_histories_id_exports_jeha_id_get_0(id, jeha_id, run_as)
If ready and available, return raw contents of exported history as a downloadable archive.

See ``PUT /api/histories/{id}/exports`` to initiate the creation of the history export - when ready, that route will return 200 status code (instead of 202) and this route can be used to download the archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**jeha_id** | [**JobExportHistoryId**](.md) | The ID of the specific Job Export History Association or `latest` (default) to download the last generated archive. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_content_api_histories_history_id_contents_id_get

> crate::models::ResponseHistoryContentApiHistoriesHistoryIdContentsIdGet history_content_api_histories_history_id_contents_id_get(history_id, id, r#type, fuzzy_count, view, keys, run_as)
Return detailed information about an HDA within a history.

Return detailed information about an `HDA` or `HDCA` within a history.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**fuzzy_count** | Option<**i32**> | This value can be used to broadly restrict the magnitude of the number of elements returned via the API for large collections. The number of actual elements returned may be \"a bit\" more than this number or \"a lot\" less - varying on the depth of nesting, balance of nesting at each level, and size of target collection. The consumer of this API should not expect a stable number or pre-calculable number of elements to be produced given this parameter - the only promise is that this API will not respond with an order of magnitude more elements estimated with this value. The UI uses this parameter to fetch a \"balanced\" concept of the \"start\" of large collections at every depth of the collection. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryContentApiHistoriesHistoryIdContentsIdGet**](Response_History_Content_Api_Histories__History_Id__Contents__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_content_api_histories_history_id_contents_id_get_0

> crate::models::ResponseHistoryContentApiHistoriesHistoryIdContentsIdGet history_content_api_histories_history_id_contents_id_get_0(history_id, id, r#type, fuzzy_count, view, keys, run_as)
Return detailed information about an HDA within a history.

Return detailed information about an `HDA` or `HDCA` within a history.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**fuzzy_count** | Option<**i32**> | This value can be used to broadly restrict the magnitude of the number of elements returned via the API for large collections. The number of actual elements returned may be \"a bit\" more than this number or \"a lot\" less - varying on the depth of nesting, balance of nesting at each level, and size of target collection. The consumer of this API should not expect a stable number or pre-calculable number of elements to be produced given this parameter - the only promise is that this API will not respond with an order of magnitude more elements estimated with this value. The UI uses this parameter to fetch a \"balanced\" concept of the \"start\" of large collections at every depth of the collection. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryContentApiHistoriesHistoryIdContentsIdGet**](Response_History_Content_Api_Histories__History_Id__Contents__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_content_typed_api_histories_history_id_contents_type_sid_get

> crate::models::ResponseHistoryContentTypedApiHistoriesHistoryIdContentsTypeSIdGet history_content_typed_api_histories_history_id_contents_type_sid_get(history_id, id, r#type, fuzzy_count, view, keys, run_as)
Return detailed information about a specific HDA or HDCA with the given `ID` within a history.

Return detailed information about an `HDA` or `HDCA` within a history.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**fuzzy_count** | Option<**i32**> | This value can be used to broadly restrict the magnitude of the number of elements returned via the API for large collections. The number of actual elements returned may be \"a bit\" more than this number or \"a lot\" less - varying on the depth of nesting, balance of nesting at each level, and size of target collection. The consumer of this API should not expect a stable number or pre-calculable number of elements to be produced given this parameter - the only promise is that this API will not respond with an order of magnitude more elements estimated with this value. The UI uses this parameter to fetch a \"balanced\" concept of the \"start\" of large collections at every depth of the collection. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryContentTypedApiHistoriesHistoryIdContentsTypeSIdGet**](Response_History_Content_Typed_Api_Histories__History_Id__Contents__Type_S__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_content_typed_api_histories_history_id_contents_type_sid_get_0

> crate::models::ResponseHistoryContentTypedApiHistoriesHistoryIdContentsTypeSIdGet history_content_typed_api_histories_history_id_contents_type_sid_get_0(history_id, id, r#type, fuzzy_count, view, keys, run_as)
Return detailed information about a specific HDA or HDCA with the given `ID` within a history.

Return detailed information about an `HDA` or `HDCA` within a history.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**fuzzy_count** | Option<**i32**> | This value can be used to broadly restrict the magnitude of the number of elements returned via the API for large collections. The number of actual elements returned may be \"a bit\" more than this number or \"a lot\" less - varying on the depth of nesting, balance of nesting at each level, and size of target collection. The consumer of this API should not expect a stable number or pre-calculable number of elements to be produced given this parameter - the only promise is that this API will not respond with an order of magnitude more elements estimated with this value. The UI uses this parameter to fetch a \"balanced\" concept of the \"start\" of large collections at every depth of the collection. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseHistoryContentTypedApiHistoriesHistoryIdContentsTypeSIdGet**](Response_History_Content_Typed_Api_Histories__History_Id__Contents__Type_S__Id__Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_contents_api_histories_history_id_contents_get

> crate::models::HistoryContentsApiHistoriesHistoryIdContentsGet200Response history_contents_api_histories_history_id_contents_get(history_id, v, details, ids, types, deleted, visible, view, keys, q, qv, offset, limit, order, run_as)
Returns the contents of the given history.

Return a list of `HDA`/`HDCA` data for the history with the given ``ID``.  - The contents can be filtered and queried using the appropriate parameters. - The amount of information returned for each item can be customized.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**v** | Option<**String**> | Only `dev` value is allowed. Set it to use the latest version of this endpoint. **All parameters marked as `deprecated` will be ignored when this parameter is set.** |  |
**details** | Option<**String**> | Legacy name for the `dataset_details` parameter. |  |
**ids** | Option<**String**> | A comma-separated list of encoded `HDA/HDCA` IDs. If this list is provided, only information about the specific datasets will be returned. Also, setting this value will return `all` details of the content item. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of kinds of contents to return (currently just `dataset` and `dataset_collection` are available). If unset, all types will be returned. |  |
**deleted** | Option<**bool**> | Whether to return deleted or undeleted datasets only. Leave unset for both. |  |
**visible** | Option<**bool**> | Whether to return visible or hidden datasets only. Leave unset for both. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HistoryContentsApiHistoriesHistoryIdContentsGet200Response**](history_contents_api_histories__history_id__contents_get_200_response.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.galaxy.history.contents.stats+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_contents_api_histories_history_id_contents_get_0

> crate::models::HistoryContentsApiHistoriesHistoryIdContentsGet200Response history_contents_api_histories_history_id_contents_get_0(history_id, v, details, ids, types, deleted, visible, view, keys, q, qv, offset, limit, order, run_as)
Returns the contents of the given history.

Return a list of `HDA`/`HDCA` data for the history with the given ``ID``.  - The contents can be filtered and queried using the appropriate parameters. - The amount of information returned for each item can be customized.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**v** | Option<**String**> | Only `dev` value is allowed. Set it to use the latest version of this endpoint. **All parameters marked as `deprecated` will be ignored when this parameter is set.** |  |
**details** | Option<**String**> | Legacy name for the `dataset_details` parameter. |  |
**ids** | Option<**String**> | A comma-separated list of encoded `HDA/HDCA` IDs. If this list is provided, only information about the specific datasets will be returned. Also, setting this value will return `all` details of the content item. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of kinds of contents to return (currently just `dataset` and `dataset_collection` are available). If unset, all types will be returned. |  |
**deleted** | Option<**bool**> | Whether to return deleted or undeleted datasets only. Leave unset for both. |  |
**visible** | Option<**bool**> | Whether to return visible or hidden datasets only. Leave unset for both. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::HistoryContentsApiHistoriesHistoryIdContentsGet200Response**](history_contents_api_histories__history_id__contents_get_200_response.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.galaxy.history.contents.stats+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## history_contents_display_api_histories_history_id_contents_history_content_id_display_get

> history_contents_display_api_histories_history_id_contents_history_content_id_display_get(history_id, history_content_id, preview, filename, to_ext, raw, run_as)
Displays (preview) or downloads dataset content.

Streams the dataset for download or the contents preview to be displayed in a browser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** |  | [required] |
**history_content_id** | **String** | The encoded database identifier of the dataset. | [required] |
**preview** | Option<**bool**> | Whether to get preview contents to be directly displayed on the web. If preview is False (default) the contents will be downloaded instead. |  |[default to false]
**filename** | Option<**String**> | TODO |  |
**to_ext** | Option<**String**> | The file extension when downloading the display data. Use the value `data` to let the server infer it from the data type. |  |
**raw** | Option<**bool**> | The query parameter 'raw' should be considered experimental and may be dropped at some point in the future without warning. Generally, data should be processed by its datatype prior to display. |  |[default to false]
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_histories_get

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> index_api_histories_get(all, deleted, q, qv, offset, limit, order, view, keys, run_as)
Returns histories for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Whether all histories from other users in this Galaxy should be included. Only admins are allowed to query all histories. |  |[default to false]
**deleted** | Option<**bool**> | Whether to return only deleted items. |  |[default to false]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_histories_get_0

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> index_api_histories_get_0(all, deleted, q, qv, offset, limit, order, view, keys, run_as)
Returns histories for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Whether all histories from other users in this Galaxy should be included. Only admins are allowed to query all histories. |  |[default to false]
**deleted** | Option<**bool**> | Whether to return only deleted items. |  |[default to false]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_histories_history_id_contents_type_s_get

> crate::models::ResponseIndexApiHistoriesHistoryIdContentsTypeSGet index_api_histories_history_id_contents_type_s_get(history_id, r#type, v, details, ids, types, deleted, visible, view, keys, q, qv, offset, limit, order, run_as)
Returns the contents of the given history filtered by type.

Return a list of `HDA`/`HDCA` data for the history with the given ``ID``.  - The contents can be filtered and queried using the appropriate parameters. - The amount of information returned for each item can be customized.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**r#type** | **String** |  | [required] |
**v** | Option<**String**> | Only `dev` value is allowed. Set it to use the latest version of this endpoint. **All parameters marked as `deprecated` will be ignored when this parameter is set.** |  |
**details** | Option<**String**> | Legacy name for the `dataset_details` parameter. |  |
**ids** | Option<**String**> | A comma-separated list of encoded `HDA/HDCA` IDs. If this list is provided, only information about the specific datasets will be returned. Also, setting this value will return `all` details of the content item. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of kinds of contents to return (currently just `dataset` and `dataset_collection` are available). If unset, all types will be returned. |  |
**deleted** | Option<**bool**> | Whether to return deleted or undeleted datasets only. Leave unset for both. |  |
**visible** | Option<**bool**> | Whether to return visible or hidden datasets only. Leave unset for both. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseIndexApiHistoriesHistoryIdContentsTypeSGet**](Response_Index_Api_Histories__History_Id__Contents__Type_S_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_histories_history_id_contents_type_s_get_0

> crate::models::ResponseIndexApiHistoriesHistoryIdContentsTypeSGet index_api_histories_history_id_contents_type_s_get_0(history_id, r#type, v, details, ids, types, deleted, visible, view, keys, q, qv, offset, limit, order, run_as)
Returns the contents of the given history filtered by type.

Return a list of `HDA`/`HDCA` data for the history with the given ``ID``.  - The contents can be filtered and queried using the appropriate parameters. - The amount of information returned for each item can be customized.  **Note**: Anonymous users are allowed to get their current history contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**r#type** | **String** |  | [required] |
**v** | Option<**String**> | Only `dev` value is allowed. Set it to use the latest version of this endpoint. **All parameters marked as `deprecated` will be ignored when this parameter is set.** |  |
**details** | Option<**String**> | Legacy name for the `dataset_details` parameter. |  |
**ids** | Option<**String**> | A comma-separated list of encoded `HDA/HDCA` IDs. If this list is provided, only information about the specific datasets will be returned. Also, setting this value will return `all` details of the content item. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list or comma-separated list of kinds of contents to return (currently just `dataset` and `dataset_collection` are available). If unset, all types will be returned. |  |
**deleted** | Option<**bool**> | Whether to return deleted or undeleted datasets only. Leave unset for both. |  |
**visible** | Option<**bool**> | Whether to return visible or hidden datasets only. Leave unset for both. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseIndexApiHistoriesHistoryIdContentsTypeSGet**](Response_Index_Api_Histories__History_Id__Contents__Type_S_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_histories_deleted_get

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> index_deleted_api_histories_deleted_get(all, q, qv, offset, limit, order, view, keys, run_as)
Returns deleted histories for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Whether all histories from other users in this Galaxy should be included. Only admins are allowed to query all histories. |  |[default to false]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_deleted_api_histories_deleted_get_0

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> index_deleted_api_histories_deleted_get_0(all, q, qv, offset, limit, order, view, keys, run_as)
Returns deleted histories for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | Whether all histories from other users in this Galaxy should be included. Only admins are allowed to query all histories. |  |[default to false]
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_exports_api_histories_id_exports_get

> Vec<crate::models::JobExportHistoryArchiveModel> index_exports_api_histories_id_exports_get(id, run_as)
Get previous history exports (to links). Effectively returns serialized JEHA objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::JobExportHistoryArchiveModel>**](JobExportHistoryArchiveModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_exports_api_histories_id_exports_get_0

> Vec<crate::models::JobExportHistoryArchiveModel> index_exports_api_histories_id_exports_get_0(id, run_as)
Get previous history exports (to links). Effectively returns serialized JEHA objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::JobExportHistoryArchiveModel>**](JobExportHistoryArchiveModel.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_jobs_summary_api_histories_history_id_jobs_summary_get

> Vec<crate::models::ResponseIndexJobsSummaryApiHistoriesHistoryIdJobsSummaryGetInner> index_jobs_summary_api_histories_history_id_jobs_summary_get(history_id, ids, types, run_as)
Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.

Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.  **Warning**: We allow anyone to fetch job state information about any object they can guess an encoded ID for - it isn't considered protected data. This keeps polling IDs as part of state calculation for large histories and collections as efficient as possible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**ids** | Option<**String**> | A comma-separated list of encoded ids of job summary objects to return - if `ids` is specified types must also be specified and have same length. |  |
**types** | Option<**String**> | A comma-separated list of type of object represented by elements in the `ids` array - any of `Job`, `ImplicitCollectionJob`, or `WorkflowInvocation`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexJobsSummaryApiHistoriesHistoryIdJobsSummaryGetInner>**](Response_Index_Jobs_Summary_Api_Histories__History_Id__Jobs_Summary_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_jobs_summary_api_histories_history_id_jobs_summary_get_0

> Vec<crate::models::ResponseIndexJobsSummaryApiHistoriesHistoryIdJobsSummaryGetInner> index_jobs_summary_api_histories_history_id_jobs_summary_get_0(history_id, ids, types, run_as)
Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.

Return job state summary info for jobs, implicit groups jobs for collections or workflow invocations.  **Warning**: We allow anyone to fetch job state information about any object they can guess an encoded ID for - it isn't considered protected data. This keeps polling IDs as part of state calculation for large histories and collections as efficient as possible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**ids** | Option<**String**> | A comma-separated list of encoded ids of job summary objects to return - if `ids` is specified types must also be specified and have same length. |  |
**types** | Option<**String**> | A comma-separated list of type of object represented by elements in the `ids` array - any of `Job`, `ImplicitCollectionJob`, or `WorkflowInvocation`. |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexJobsSummaryApiHistoriesHistoryIdJobsSummaryGetInner>**](Response_Index_Jobs_Summary_Api_Histories__History_Id__Jobs_Summary_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post

> crate::models::AsyncTaskResultSummary materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post(history_id, id, run_as)
Materialize a deferred dataset into real, usable dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post_0

> crate::models::AsyncTaskResultSummary materialize_dataset_api_histories_history_id_contents_datasets_id_materialize_post_0(history_id, id, run_as)
Materialize a deferred dataset into real, usable dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materialize_to_history_api_histories_history_id_materialize_post

> crate::models::AsyncTaskResultSummary materialize_to_history_api_histories_history_id_materialize_post(history_id, materialize_dataset_instance_api_request, run_as)
Materialize a deferred library or HDA dataset into real, usable dataset in specified history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**materialize_dataset_instance_api_request** | [**MaterializeDatasetInstanceApiRequest**](MaterializeDatasetInstanceApiRequest.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## materialize_to_history_api_histories_history_id_materialize_post_0

> crate::models::AsyncTaskResultSummary materialize_to_history_api_histories_history_id_materialize_post_0(history_id, materialize_dataset_instance_api_request, run_as)
Materialize a deferred library or HDA dataset into real, usable dataset in specified history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**materialize_dataset_instance_api_request** | [**MaterializeDatasetInstanceApiRequest**](MaterializeDatasetInstanceApiRequest.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_collection_download_api_dataset_collections_id_prepare_download_post

> crate::models::AsyncFile prepare_collection_download_api_dataset_collections_id_prepare_download_post(history_id, id, run_as)
Prepare an short term storage object that the collection will be downloaded to.

The history dataset collection will be written as a `zip` archive to the returned short term storage object. Progress tracking this file's creation can be tracked with the short_term_storage API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_collection_download_api_dataset_collections_id_prepare_download_post_0

> crate::models::AsyncFile prepare_collection_download_api_dataset_collections_id_prepare_download_post_0(history_id, id, run_as)
Prepare an short term storage object that the collection will be downloaded to.

The history dataset collection will be written as a `zip` archive to the returned short term storage object. Progress tracking this file's creation can be tracked with the short_term_storage API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post

> crate::models::AsyncFile prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post(history_id, id, run_as)
Prepare an short term storage object that the collection will be downloaded to.

The history dataset collection will be written as a `zip` archive to the returned short term storage object. Progress tracking this file's creation can be tracked with the short_term_storage API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post_0

> crate::models::AsyncFile prepare_collection_download_api_histories_history_id_contents_dataset_collections_id_prepare_download_post_0(history_id, id, run_as)
Prepare an short term storage object that the collection will be downloaded to.

The history dataset collection will be written as a `zip` archive to the returned short term storage object. Progress tracking this file's creation can be tracked with the short_term_storage API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the `HDCA` contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post

> crate::models::AsyncFile prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post(history_id, id, r#type, store_export_payload, run_as)
Prepare a dataset or dataset collection for export-style download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**store_export_payload** | [**StoreExportPayload**](StoreExportPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post_0

> crate::models::AsyncFile prepare_store_download_api_histories_history_id_contents_type_sid_prepare_store_download_post_0(history_id, id, r#type, store_export_payload, run_as)
Prepare a dataset or dataset collection for export-style download.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**store_export_payload** | [**StoreExportPayload**](StoreExportPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_histories_id_prepare_store_download_post

> crate::models::AsyncFile prepare_store_download_api_histories_id_prepare_store_download_post(id, store_export_payload, run_as)
Return a short term storage token to monitor download of the history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**store_export_payload** | [**StoreExportPayload**](StoreExportPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_store_download_api_histories_id_prepare_store_download_post_0

> crate::models::AsyncFile prepare_store_download_api_histories_id_prepare_store_download_post_0(id, store_export_payload, run_as)
Return a short term storage token to monitor download of the history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**store_export_payload** | [**StoreExportPayload**](StoreExportPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncFile**](AsyncFile.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_histories_id_publish_put

> crate::models::SharingStatus publish_api_histories_id_publish_put(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_api_histories_id_publish_put_0

> crate::models::SharingStatus publish_api_histories_id_publish_put_0(id, run_as)
Makes this item public and accessible by a URL link.

Makes this item publicly available by a URL link and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_api_histories_published_get

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> published_api_histories_published_get(q, qv, offset, limit, order, view, keys, run_as)
Return all histories that are published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_api_histories_published_get_0

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> published_api_histories_published_get_0(q, qv, offset, limit, order, view, keys, run_as)
Return all histories that are published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_histories_id_slug_put

> set_slug_api_histories_id_slug_put(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**set_slug_payload** | [**SetSlugPayload**](SetSlugPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_slug_api_histories_id_slug_put_0

> set_slug_api_histories_id_slug_put_0(id, set_slug_payload, run_as)
Set a new slug for this shared item.

Sets a new slug to access this item by URL. The new slug must be unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**set_slug_payload** | [**SetSlugPayload**](SetSlugPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

 (empty response body)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_with_users_api_histories_id_share_with_users_put

> crate::models::ShareWithStatus share_with_users_api_histories_id_share_with_users_put(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**share_with_payload** | [**ShareWithPayload**](ShareWithPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ShareWithStatus**](ShareWithStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_with_users_api_histories_id_share_with_users_put_0

> crate::models::ShareWithStatus share_with_users_api_histories_id_share_with_users_put_0(id, share_with_payload, run_as)
Share this item with specific users.

Shares this item with specific users and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**share_with_payload** | [**ShareWithPayload**](ShareWithPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ShareWithStatus**](ShareWithStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shared_with_me_api_histories_shared_with_me_get

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> shared_with_me_api_histories_shared_with_me_get(q, qv, offset, limit, order, view, keys, run_as)
Return all histories that are shared with the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shared_with_me_api_histories_shared_with_me_get_0

> Vec<crate::models::ResponseIndexApiHistoriesGetInner> shared_with_me_api_histories_shared_with_me_get_0(q, qv, offset, limit, order, view, keys, run_as)
Return all histories that are shared with the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<[**Vec<String>**](String.md)> | Generally a property name to filter by followed by an (often optional) hyphen and operator string. |  |
**qv** | Option<[**Vec<String>**](String.md)> | The value to filter by. |  |
**offset** | Option<**i32**> | Starts at the beginning skip the first ( offset - 1 ) items and begin returning at the Nth item |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**order** | Option<**String**> | String containing one of the valid ordering attributes followed (optionally) by '-asc' or '-dsc' for ascending and descending order respectively. Orders can be stacked as a comma-separated list of values. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiHistoriesGetInner>**](Response_Index_Api_Histories_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_histories_id_sharing_get

> crate::models::SharingStatus sharing_api_histories_id_sharing_get(id, run_as)
Get the current sharing status of the given item.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharing_api_histories_id_sharing_get_0

> crate::models::SharingStatus sharing_api_histories_id_sharing_get_0(id, run_as)
Get the current sharing status of the given item.

Return the sharing status of the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get

> crate::models::ResponseShowJobsSummaryApiHistoriesHistoryIdContentsTypeSIdJobsSummaryGet show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get(history_id, id, r#type, run_as)
Return detailed information about an `HDA` or `HDCAs` jobs.

Return detailed information about an `HDA` or `HDCAs` jobs.  **Warning**: We allow anyone to fetch job state information about any object they can guess an encoded ID for - it isn't considered protected data. This keeps polling IDs as part of state calculation for large histories and collections as efficient as possible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowJobsSummaryApiHistoriesHistoryIdContentsTypeSIdJobsSummaryGet**](Response_Show_Jobs_Summary_Api_Histories__History_Id__Contents__Type_S__Id__Jobs_Summary_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get_0

> crate::models::ResponseShowJobsSummaryApiHistoriesHistoryIdContentsTypeSIdJobsSummaryGet show_jobs_summary_api_histories_history_id_contents_type_sid_jobs_summary_get_0(history_id, id, r#type, run_as)
Return detailed information about an `HDA` or `HDCAs` jobs.

Return detailed information about an `HDA` or `HDCAs` jobs.  **Warning**: We allow anyone to fetch job state information about any object they can guess an encoded ID for - it isn't considered protected data. This keeps polling IDs as part of state calculation for large histories and collections as efficient as possible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowJobsSummaryApiHistoriesHistoryIdContentsTypeSIdJobsSummaryGet**](Response_Show_Jobs_Summary_Api_Histories__History_Id__Contents__Type_S__Id__Jobs_Summary_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_recent_api_histories_most_recently_used_get

> crate::models::ResponseShowRecentApiHistoriesMostRecentlyUsedGet show_recent_api_histories_most_recently_used_get(view, keys, run_as)
Returns the most recently used history of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowRecentApiHistoriesMostRecentlyUsedGet**](Response_Show_Recent_Api_Histories_Most_Recently_Used_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_recent_api_histories_most_recently_used_get_0

> crate::models::ResponseShowRecentApiHistoriesMostRecentlyUsedGet show_recent_api_histories_most_recently_used_get_0(view, keys, run_as)
Returns the most recently used history of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseShowRecentApiHistoriesMostRecentlyUsedGet**](Response_Show_Recent_Api_Histories_Most_Recently_Used_Get.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_api_histories_deleted_id_undelete_post

> crate::models::ResponseUndeleteApiHistoriesDeletedIdUndeletePost undelete_api_histories_deleted_id_undelete_post(id, view, keys, run_as)
Restores a deleted history with the given ID (that hasn't been purged).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUndeleteApiHistoriesDeletedIdUndeletePost**](Response_Undelete_Api_Histories_Deleted__Id__Undelete_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_api_histories_deleted_id_undelete_post_0

> crate::models::ResponseUndeleteApiHistoriesDeletedIdUndeletePost undelete_api_histories_deleted_id_undelete_post_0(id, view, keys, run_as)
Restores a deleted history with the given ID (that hasn't been purged).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUndeleteApiHistoriesDeletedIdUndeletePost**](Response_Undelete_Api_Histories_Deleted__Id__Undelete_Post.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_histories_id_unpublish_put

> crate::models::SharingStatus unpublish_api_histories_id_unpublish_put(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_api_histories_id_unpublish_put_0

> crate::models::SharingStatus unpublish_api_histories_id_unpublish_put_0(id, run_as)
Removes this item from the published list.

Removes this item from the published list and return the current sharing status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::SharingStatus**](SharingStatus.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_history_id_contents_id_put

> crate::models::ResponseUpdateApiHistoriesHistoryIdContentsIdPut update_api_histories_history_id_contents_id_put(history_id, id, body, r#type, view, keys, run_as)
Updates the values for the history content item with the given ``ID``.

Updates the values for the history content item with the given ``ID``.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**body** | **serde_json::Value** |  | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesHistoryIdContentsIdPut**](Response_Update_Api_Histories__History_Id__Contents__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_history_id_contents_id_put_0

> crate::models::ResponseUpdateApiHistoriesHistoryIdContentsIdPut update_api_histories_history_id_contents_id_put_0(history_id, id, body, r#type, view, keys, run_as)
Updates the values for the history content item with the given ``ID``.

Updates the values for the history content item with the given ``ID``.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**body** | **serde_json::Value** |  | [required] |
**r#type** | Option<[**crate::models::HistoryContentType**](.md)> | The type of the history element to show. |  |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesHistoryIdContentsIdPut**](Response_Update_Api_Histories__History_Id__Contents__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_history_id_contents_type_sid_put

> crate::models::ResponseUpdateApiHistoriesHistoryIdContentsTypeSIdPut update_api_histories_history_id_contents_type_sid_put(history_id, id, r#type, body, view, keys, run_as)
Updates the values for the history content item with the given ``ID``.

Updates the values for the history content item with the given ``ID``.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**body** | **serde_json::Value** |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesHistoryIdContentsTypeSIdPut**](Response_Update_Api_Histories__History_Id__Contents__Type_S__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_history_id_contents_type_sid_put_0

> crate::models::ResponseUpdateApiHistoriesHistoryIdContentsTypeSIdPut update_api_histories_history_id_contents_type_sid_put_0(history_id, id, r#type, body, view, keys, run_as)
Updates the values for the history content item with the given ``ID``.

Updates the values for the history content item with the given ``ID``.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**body** | **serde_json::Value** |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesHistoryIdContentsTypeSIdPut**](Response_Update_Api_Histories__History_Id__Contents__Type_S__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_id_put

> crate::models::ResponseUpdateApiHistoriesIdPut update_api_histories_id_put(id, body, view, keys, run_as)
Updates the values for the history with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesIdPut**](Response_Update_Api_Histories__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_histories_id_put_0

> crate::models::ResponseUpdateApiHistoriesIdPut update_api_histories_id_put_0(id, body, view, keys, run_as)
Updates the values for the history with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::ResponseUpdateApiHistoriesIdPut**](Response_Update_Api_Histories__Id__Put.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_batch_api_histories_history_id_contents_put

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> update_batch_api_histories_history_id_contents_put(history_id, update_history_contents_batch_payload, view, keys, run_as)
Batch update specific properties of a set items contained in the given History.

Batch update specific properties of a set items contained in the given History.  If you provide an invalid/unknown property key the request will not fail, but no changes will be made to the items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**update_history_contents_batch_payload** | [**UpdateHistoryContentsBatchPayload**](UpdateHistoryContentsBatchPayload.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_batch_api_histories_history_id_contents_put_0

> Vec<crate::models::ResponseIndexApiDatasetsGetInner> update_batch_api_histories_history_id_contents_put_0(history_id, update_history_contents_batch_payload, view, keys, run_as)
Batch update specific properties of a set items contained in the given History.

Batch update specific properties of a set items contained in the given History.  If you provide an invalid/unknown property key the request will not fail, but no changes will be made to the items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**update_history_contents_batch_payload** | [**UpdateHistoryContentsBatchPayload**](UpdateHistoryContentsBatchPayload.md) |  | [required] |
**view** | Option<**String**> | View to be passed to the serializer |  |
**keys** | Option<**String**> | Comma-separated list of keys to be passed to the serializer |  |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**Vec<crate::models::ResponseIndexApiDatasetsGetInner>**](Response_Index_Api_Datasets_Get_inner.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permissions_api_histories_history_id_contents_dataset_id_permissions_put

> crate::models::DatasetAssociationRoles update_permissions_api_histories_history_id_contents_dataset_id_permissions_put(history_id, dataset_id, body, run_as)
Set permissions of the given history dataset to the given role ids.

Set permissions of the given history dataset to the given role ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**dataset_id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**body** | **serde_json::Value** |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::DatasetAssociationRoles**](DatasetAssociationRoles.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permissions_api_histories_history_id_contents_dataset_id_permissions_put_0

> crate::models::DatasetAssociationRoles update_permissions_api_histories_history_id_contents_dataset_id_permissions_put_0(history_id, dataset_id, body, run_as)
Set permissions of the given history dataset to the given role ids.

Set permissions of the given history dataset to the given role ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**dataset_id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**body** | **serde_json::Value** |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::DatasetAssociationRoles**](DatasetAssociationRoles.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_api_histories_history_id_contents_id_validate_put

> serde_json::Value validate_api_histories_history_id_contents_id_validate_put(history_id, id, run_as)
Validates the metadata associated with a dataset within a History.

Validates the metadata associated with a dataset within a History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_api_histories_history_id_contents_id_validate_put_0

> serde_json::Value validate_api_histories_history_id_contents_id_validate_put_0(history_id, id, run_as)
Validates the metadata associated with a dataset within a History.

Validates the metadata associated with a dataset within a History.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_histories_history_id_contents_type_sid_write_store_post

> crate::models::AsyncTaskResultSummary write_store_api_histories_history_id_contents_type_sid_write_store_post(history_id, id, r#type, write_store_to_payload, run_as)
Prepare a dataset or dataset collection for export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_histories_history_id_contents_type_sid_write_store_post_0

> crate::models::AsyncTaskResultSummary write_store_api_histories_history_id_contents_type_sid_write_store_post_0(history_id, id, r#type, write_store_to_payload, run_as)
Prepare a dataset or dataset collection for export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**history_id** | **String** | The ID of the History. | [required] |
**id** | **String** | The ID of the item (`HDA`/`HDCA`) contained in the history. | [required] |
**r#type** | [**HistoryContentType**](.md) |  | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_histories_id_write_store_post

> crate::models::AsyncTaskResultSummary write_store_api_histories_id_write_store_post(id, write_store_to_payload, run_as)
Prepare history for export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_store_api_histories_id_write_store_post_0

> crate::models::AsyncTaskResultSummary write_store_api_histories_id_write_store_post_0(id, write_store_to_payload, run_as)
Prepare history for export-style download and write to supplied URI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The encoded database identifier of the History. | [required] |
**write_store_to_payload** | [**WriteStoreToPayload**](WriteStoreToPayload.md) |  | [required] |
**run_as** | Option<**String**> | The user ID that will be used to effectively make this API call. Only admins and designated users can make API calls on behalf of other users. |  |

### Return type

[**crate::models::AsyncTaskResultSummary**](AsyncTaskResultSummary.md)

### Authorization

[APIKeyCookie](../README.md#APIKeyCookie), [APIKeyHeader](../README.md#APIKeyHeader), [APIKeyQuery](../README.md#APIKeyQuery)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

