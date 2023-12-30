# \MeteringApi

All URIs are relative to *https://api.saasus.io/v1/pricing*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_metering_unit**](MeteringApi.md#create_metering_unit) | **Post** /metering/units | メータリングユニットの作成(Create Metering Unit)
[**delete_metering_unit_by_id**](MeteringApi.md#delete_metering_unit_by_id) | **Delete** /metering/units/{metering_unit_id} | メータリングユニットを削除(Delete Metering Unit)
[**delete_metering_unit_timestamp_count**](MeteringApi.md#delete_metering_unit_timestamp_count) | **Delete** /metering/tenants/{tenant_id}/units/{metering_unit_name}/timestamp/{timestamp} | 指定したタイムスタンプのメータリングユニットカウントを削除(Delete Metering Uunit Count for Specified Timestamp)
[**get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date**](MeteringApi.md#get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date) | **Get** /metering/tenants/{tenant_id}/units/{metering_unit_name}/date/{date} | 指定した日付のメータリングユニットカウントを取得(Get Metering Unit Count for Specific Date)
[**get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date_period**](MeteringApi.md#get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date_period) | **Get** /metering/tenants/{tenant_id}/units/{metering_unit_name}/date-period | 指定した日時期間のメータリングユニットカウントを取得(Obtain metering unit counts for a specified date/time period)
[**get_metering_unit_date_count_by_tenant_id_and_unit_name_today**](MeteringApi.md#get_metering_unit_date_count_by_tenant_id_and_unit_name_today) | **Get** /metering/tenants/{tenant_id}/units/{metering_unit_name}/today | 当日のメータリングユニットカウントを取得(Get Metering Unit Count for the Current Day)
[**get_metering_unit_date_counts_by_tenant_id_and_date**](MeteringApi.md#get_metering_unit_date_counts_by_tenant_id_and_date) | **Get** /metering/tenants/{tenant_id}/units/date/{date} | 指定日の全メータリングユニットカウントを取得(Get All Metering Unit Counts for a Specified Date)
[**get_metering_unit_month_count_by_tenant_id_and_unit_name_and_month**](MeteringApi.md#get_metering_unit_month_count_by_tenant_id_and_unit_name_and_month) | **Get** /metering/tenants/{tenant_id}/units/{metering_unit_name}/month/{month} | 指定月のメータリングユニットカウントを取得(Get the Metering Unit Count for the Specified Month)
[**get_metering_unit_month_count_by_tenant_id_and_unit_name_this_month**](MeteringApi.md#get_metering_unit_month_count_by_tenant_id_and_unit_name_this_month) | **Get** /metering/tenants/{tenant_id}/units/{metering_unit_name}/thismonth | 当月のメータリングユニットカウントを取得(Get Metering Unit Count for the Current Month)
[**get_metering_unit_month_counts_by_tenant_id_and_month**](MeteringApi.md#get_metering_unit_month_counts_by_tenant_id_and_month) | **Get** /metering/tenants/{tenant_id}/units/month/{month} | 指定月の全メータリングユニットカウントを取得(Get All Metering Unit Counts for the Specified Month)
[**get_metering_units**](MeteringApi.md#get_metering_units) | **Get** /metering/units | メータリングユニットを取得(Get all metering units)
[**update_metering_unit_by_id**](MeteringApi.md#update_metering_unit_by_id) | **Patch** /metering/units/{metering_unit_id} | メータリングユニットを更新(Update Metering Unit)
[**update_metering_unit_timestamp_count**](MeteringApi.md#update_metering_unit_timestamp_count) | **Put** /metering/tenants/{tenant_id}/units/{metering_unit_name}/timestamp/{timestamp} | 指定したタイムスタンプのメータリングユニットカウントを更新(Update Metering Unit Count for Specified Timestamp)
[**update_metering_unit_timestamp_count_now**](MeteringApi.md#update_metering_unit_timestamp_count_now) | **Put** /metering/tenants/{tenant_id}/units/{metering_unit_name}/now | 現在時刻のメータリングユニットカウントを更新(Update Metering Unit Count for Current Time)



## create_metering_unit

> crate::models::MeteringUnit create_metering_unit(body)
メータリングユニットの作成(Create Metering Unit)

メータリングユニットを作成します。  Create a metering unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**crate::models::MeteringUnitProps**> |  |  |

### Return type

[**crate::models::MeteringUnit**](MeteringUnit.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metering_unit_by_id

> delete_metering_unit_by_id(metering_unit_id)
メータリングユニットを削除(Delete Metering Unit)

メータリングユニットを削除します。  Delete metering unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metering_unit_id** | **String** | メータリングユニットID(metering unit id) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metering_unit_timestamp_count

> delete_metering_unit_timestamp_count(tenant_id, metering_unit_name, timestamp)
指定したタイムスタンプのメータリングユニットカウントを削除(Delete Metering Uunit Count for Specified Timestamp)

指定したタイムスタンプのメータリングユニットカウントを削除します。  Deletes metering unit count for the specified timestamp. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**timestamp** | **i32** | タイムスタンプ(timestamp) | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date

> crate::models::MeteringUnitDateCount get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date(tenant_id, metering_unit_name, date)
指定した日付のメータリングユニットカウントを取得(Get Metering Unit Count for Specific Date)

指定した日付のメータリングユニットカウントを取得します。  Gets the metering unit count for specific date. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**date** | **String** | 日(date) | [required] |

### Return type

[**crate::models::MeteringUnitDateCount**](MeteringUnitDateCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date_period

> crate::models::MeteringUnitDatePeriodCounts get_metering_unit_date_count_by_tenant_id_and_unit_name_and_date_period(tenant_id, metering_unit_name, start_timestamp, end_timestamp)
指定した日時期間のメータリングユニットカウントを取得(Obtain metering unit counts for a specified date/time period)

指定した日時期間のメータリングユニットカウントを取得します。  Obtain metering unit counts for a specified date/time period. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**start_timestamp** | Option<**i32**> | 開始日時(timestamp) |  |
**end_timestamp** | Option<**i32**> | 終了日時(timestamp) |  |

### Return type

[**crate::models::MeteringUnitDatePeriodCounts**](MeteringUnitDatePeriodCounts.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_date_count_by_tenant_id_and_unit_name_today

> crate::models::MeteringUnitDateCount get_metering_unit_date_count_by_tenant_id_and_unit_name_today(tenant_id, metering_unit_name)
当日のメータリングユニットカウントを取得(Get Metering Unit Count for the Current Day)

当日のメータリングユニットカウントを取得します。  Get the metering unit count for the current day. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |

### Return type

[**crate::models::MeteringUnitDateCount**](MeteringUnitDateCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_date_counts_by_tenant_id_and_date

> crate::models::MeteringUnitDateCounts get_metering_unit_date_counts_by_tenant_id_and_date(tenant_id, date)
指定日の全メータリングユニットカウントを取得(Get All Metering Unit Counts for a Specified Date)

指定した日の全メータリングユニットカウントを取得します。  Gets the total metering unit count for the specified date. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**date** | **String** | 日(date) | [required] |

### Return type

[**crate::models::MeteringUnitDateCounts**](MeteringUnitDateCounts.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_month_count_by_tenant_id_and_unit_name_and_month

> crate::models::MeteringUnitMonthCount get_metering_unit_month_count_by_tenant_id_and_unit_name_and_month(tenant_id, metering_unit_name, month)
指定月のメータリングユニットカウントを取得(Get the Metering Unit Count for the Specified Month)

指定した月のメータリングユニットカウントを取得します。  Gets the metering unit count for the specified month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**month** | **String** | 月(month) | [required] |

### Return type

[**crate::models::MeteringUnitMonthCount**](MeteringUnitMonthCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_month_count_by_tenant_id_and_unit_name_this_month

> crate::models::MeteringUnitMonthCount get_metering_unit_month_count_by_tenant_id_and_unit_name_this_month(tenant_id, metering_unit_name)
当月のメータリングユニットカウントを取得(Get Metering Unit Count for the Current Month)

当月のメータリングユニットカウントを取得します。  Get the metering unit count for the current month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |

### Return type

[**crate::models::MeteringUnitMonthCount**](MeteringUnitMonthCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_unit_month_counts_by_tenant_id_and_month

> crate::models::MeteringUnitMonthCounts get_metering_unit_month_counts_by_tenant_id_and_month(tenant_id, month)
指定月の全メータリングユニットカウントを取得(Get All Metering Unit Counts for the Specified Month)

指定した月の全メータリングユニットカウントを取得します。  Gets all metering unit counts for the specified month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**month** | **String** | 月(month) | [required] |

### Return type

[**crate::models::MeteringUnitMonthCounts**](MeteringUnitMonthCounts.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metering_units

> crate::models::MeteringUnits get_metering_units()
メータリングユニットを取得(Get all metering units)

全てのメータリングユニットを取得します。  Get all metering units. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MeteringUnits**](MeteringUnits.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metering_unit_by_id

> update_metering_unit_by_id(metering_unit_id, body)
メータリングユニットを更新(Update Metering Unit)

メータリングユニットを更新します。  Update metering unit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metering_unit_id** | **String** | メータリングユニットID(metering unit id) | [required] |
**body** | Option<**crate::models::MeteringUnitProps**> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metering_unit_timestamp_count

> crate::models::MeteringUnitTimestampCount update_metering_unit_timestamp_count(tenant_id, metering_unit_name, timestamp, update_metering_unit_timestamp_count_param)
指定したタイムスタンプのメータリングユニットカウントを更新(Update Metering Unit Count for Specified Timestamp)

指定したタイムスタンプのメータリングユニットカウントを更新します。  Update metering unit count for the specified timestamp. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**timestamp** | **i32** | タイムスタンプ(timestamp) | [required] |
**update_metering_unit_timestamp_count_param** | Option<[**UpdateMeteringUnitTimestampCountParam**](UpdateMeteringUnitTimestampCountParam.md)> |  |  |

### Return type

[**crate::models::MeteringUnitTimestampCount**](MeteringUnitTimestampCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metering_unit_timestamp_count_now

> crate::models::MeteringUnitTimestampCount update_metering_unit_timestamp_count_now(tenant_id, metering_unit_name, update_metering_unit_timestamp_count_now_param)
現在時刻のメータリングユニットカウントを更新(Update Metering Unit Count for Current Time)

現在時刻のメータリングユニットカウントを更新します。  Update the metering unit count for the current time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | テナントID(tenant id) | [required] |
**metering_unit_name** | **String** | 計測ユニット名(metering unit name) | [required] |
**update_metering_unit_timestamp_count_now_param** | Option<[**UpdateMeteringUnitTimestampCountNowParam**](UpdateMeteringUnitTimestampCountNowParam.md)> |  |  |

### Return type

[**crate::models::MeteringUnitTimestampCount**](MeteringUnitTimestampCount.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

