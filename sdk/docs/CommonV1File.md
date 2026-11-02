# CommonV1File

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**project_id** | Option<**String**> |  | [optional]
**format** | Option<**String**> | format describes the artifact currently served at url, not the uploaded source,  and is only meaningful once status is FILE_STATUS_READY. Video flips from \"mp4\"  to \"hls\" when packaging completes; that is the only signal for choosing a  progressive player over an HLS one. Known values: jpg, png, webp, avif, gif,  mp4, hls, srt, vtt. | [optional]
**url** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**size** | Option<**String**> | size is the uploaded source's byte size for usage accounting; it is not updated  when packaging replaces the artifact, so it does not describe the object at url. | [optional]
**status** | Option<[**models::CommonV1FileStatus**](CommonV1FileStatus.md)> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | A point in time in RFC 3339 format, with up to nanosecond precision. Output uses UTC (`Z`); input may use an offset from UTC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


