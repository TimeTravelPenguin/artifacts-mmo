# \NpcsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_npcs_npcs_get**](NpcsApi.md#get_all_npcs_npcs_get) | **GET** /npcs | Get All Npcs
[**get_npc_items_npcs_code_items_get**](NpcsApi.md#get_npc_items_npcs_code_items_get) | **GET** /npcs/{code}/items | Get Npc Items
[**get_npc_npcs_code_get**](NpcsApi.md#get_npc_npcs_code_get) | **GET** /npcs/{code} | Get Npc



## get_all_npcs_npcs_get

> models::DataPageNpcSchema get_all_npcs_npcs_get(r#type, page, size)
Get All Npcs

Fetch NPCs details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<[**models::NpcType**](.md)> | The type of the NPC. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageNpcSchema**](DataPage_NPCSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_npc_items_npcs_code_items_get

> models::DataPageNpcItem get_npc_items_npcs_code_items_get(code, page, size)
Get Npc Items

Retrieve the items list of a NPC. If the NPC has items to buy or sell, they will be displayed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the NPC. | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageNpcItem**](DataPage_NPCItem_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_npc_npcs_code_get

> models::NpcResponseSchema get_npc_npcs_code_get(code)
Get Npc

Retrieve the details of a NPC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the NPC. | [required] |

### Return type

[**models::NpcResponseSchema**](NPCResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

