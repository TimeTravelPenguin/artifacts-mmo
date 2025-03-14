# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account_accounts_create_post**](AccountsApi.md#create_account_accounts_create_post) | **POST** /accounts/create | Create Account
[**get_account_accounts_account_get**](AccountsApi.md#get_account_accounts_account_get) | **GET** /accounts/{account} | Get Account
[**get_account_achievements_accounts_account_achievements_get**](AccountsApi.md#get_account_achievements_accounts_account_achievements_get) | **GET** /accounts/{account}/achievements | Get Account Achievements



## create_account_accounts_create_post

> models::ResponseSchema create_account_accounts_create_post(add_account_schema)
Create Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_account_schema** | [**AddAccountSchema**](AddAccountSchema.md) |  | [required] |

### Return type

[**models::ResponseSchema**](ResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_accounts_account_get

> models::AccountDetailsSchema get_account_accounts_account_get(account)
Get Account

Retrieve the details of a character.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | **String** | The account name. | [required] |

### Return type

[**models::AccountDetailsSchema**](AccountDetailsSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_achievements_accounts_account_achievements_get

> models::DataPageAccountAchievementSchema get_account_achievements_accounts_account_achievements_get(account, r#type, completed, page, size)
Get Account Achievements

Retrieve the achievements of a account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account** | **String** | The character name. | [required] |
**r#type** | Option<**String**> | Type of achievements. |  |
**completed** | Option<**bool**> | Filter by completed achievements. |  |
**page** | Option<**i32**> | Page number |  |[default to 1]
**size** | Option<**i32**> | Page size |  |[default to 50]

### Return type

[**models::DataPageAccountAchievementSchema**](DataPage_AccountAchievementSchema_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

