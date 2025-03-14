# \ItemsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_items_items_get**](ItemsApi.md#get_all_items_items_get) | **GET** /items | Get All Items
[**get_item_items_code_get**](ItemsApi.md#get_item_items_code_get) | **GET** /items/{code} | Get Item



## get_all_items_items_get

> models::DataPageItemSchema get_all_items_items_get(min_level, max_level, name, r#type, craft_skill, craft_material, page, size)
Get All Items

Fetch items details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_level** | Option<**i32**> | Minimum level items. |  |
**max_level** | Option<**i32**> | Maximum level items. |  |
**name** | Option<**String**> | Name of the item. |  |
**r#type** | Option<[**models::ItemType**](.md)> | Type of items. |  |
**craft_skill** | Option<[**models::CraftSkill**](.md)> | Skill to craft items. |  |
**craft_material** | Option<**String**> | Item code of items used as material for crafting. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageItemSchema**](DataPage_ItemSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_items_code_get

> models::ItemResponseSchema get_item_items_code_get(code)
Get Item

Retrieve the details of a item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the item. | [required] |

### Return type

[**models::ItemResponseSchema**](ItemResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

