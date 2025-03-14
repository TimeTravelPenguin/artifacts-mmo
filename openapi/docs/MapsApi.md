# \MapsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_maps_maps_get**](MapsApi.md#get_all_maps_maps_get) | **GET** /maps | Get All Maps
[**get_map_maps_xy_get**](MapsApi.md#get_map_maps_xy_get) | **GET** /maps/{x}/{y} | Get Map



## get_all_maps_maps_get

> models::DataPageMapSchema get_all_maps_maps_get(content_type, content_code, page, size)
Get All Maps

Fetch maps details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | Option<[**models::MapContentType**](.md)> | Type of content on the map. |  |
**content_code** | Option<**String**> | Content code on the map. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageMapSchema**](DataPage_MapSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_map_maps_xy_get

> models::MapResponseSchema get_map_maps_xy_get(x, y)
Get Map

Retrieve the details of a map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x** | **i32** | The position x of the map. | [required] |
**y** | **i32** | The position X of the map. | [required] |

### Return type

[**models::MapResponseSchema**](MapResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

