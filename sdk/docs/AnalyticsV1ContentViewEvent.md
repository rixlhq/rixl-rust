# AnalyticsV1ContentViewEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_id** | Option<**String**> |  | [optional]
**content_type** | Option<[**models::AnalyticscommonV1MediaType**](AnalyticscommonV1MediaType.md)> | enum.defined_only = true  | [optional]
**feed_id** | Option<**String**> |  | [optional]
**post_id** | Option<**String**> |  | [optional]
**view_type** | Option<**ViewType**> |  (enum: start, watch, end) | [optional]
**watch_duration_ms** | Option<**String**> | A signed duration in seconds, with up to nine fractional digits and an `s` suffix (for example, `3s` or `-0.001s`). | [optional]
**page** | Option<**Page**> |  (enum: profile, feed, standalone) | [optional]
**video_position_ms** | Option<**String**> | A signed duration in seconds, with up to nine fractional digits and an `s` suffix (for example, `3s` or `-0.001s`). | [optional]
**video_total_duration_ms** | Option<**String**> | A signed duration in seconds, with up to nine fractional digits and an `s` suffix (for example, `3s` or `-0.001s`). | [optional]
**segments** | Option<[**Vec<models::AnalyticsV1Segment>**](AnalyticsV1Segment.md)> |  | [optional]
**device_id** | Option<**String**> |  | [optional]
**country** | Option<**String**> |  | [optional]
**subtitle_language** | Option<**String**> |  | [optional]
**audio_language** | Option<**String**> |  | [optional]
**quality** | Option<[**models::AnalyticscommonV1VideoQuality**](AnalyticscommonV1VideoQuality.md)> | enum.defined_only = true  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


