#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use chrono::{Duration, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    /// Denotes whether the audio track will be played by default in a player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,

    /// A string to uniquely identify the track amongst other audio track labels for the
    /// specified video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Specifies the processing status of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AudioStatus>,

    /// A Cloudflare-generated unique identifier for a media item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackCopyParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A string to uniquely identify the track amongst other audio track labels for the
    /// specified video.
    pub label: String,

    /// An audio track URL. The server must be publicly routable and support `HTTP HEAD`
    /// requests and `HTTP GET` range requests. The server should respond to `HTTP HEAD`
    /// requests with a `content-range` header that includes the size of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackEditParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Denotes whether the audio track will be played by default in a player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// A string to uniquely identify the track amongst other audio track labels for the
    /// specified video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caption {
    /// Whether the caption was generated via AI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<bool>,

    /// The language label displayed in the native language to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The language tag in BCP 47 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// The status of a generated caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CaptionStatus>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionLanguageNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionLanguageUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The WebVTT file containing the caption or subtitle content.
    pub file: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionLanguageDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionLanguageGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionLanguageVttGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<Vec<String>>,

    /// The unique video identifier (UID).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clippedFromVideoUID")]
    pub clipped_from_video_uid: Option<String>,

    /// The date and time the clip was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// Specifies the end time for the video clip in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTimeSeconds")]
    pub end_time_seconds: Option<i64>,

    /// The maximum duration in seconds for a video upload. Can be set for a video that
    /// is not yet uploaded to limit its duration. Uploads that exceed the specified
    /// duration will fail during processing. A value of `-1` means the value is
    /// unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurationSeconds")]
    pub max_duration_seconds: Option<i64>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// The date and time the live input was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback: Option<ClipPlayback>,

    /// The video's preview page URI. This field is omitted until encoding is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<bool>,

    /// Specifies the start time for the video clip in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTimeSeconds")]
    pub start_time_seconds: Option<i64>,

    /// Specifies the processing status for all quality levels for a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClipStatus>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<ClipWatermark>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The unique video identifier (UID).
    #[serde(rename = "clippedFromVideoUID")]
    pub clipped_from_video_uid: String,

    /// Specifies the end time for the video clip in seconds.
    #[serde(rename = "endTimeSeconds")]
    pub end_time_seconds: String,

    /// Specifies the start time for the video clip in seconds.
    #[serde(rename = "startTimeSeconds")]
    pub start_time_seconds: String,

    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// The maximum duration in seconds for a video upload. Can be set for a video that
    /// is not yet uploaded to limit its duration. Uploads that exceed the specified
    /// duration will fail during processing. A value of `-1` means the value is
    /// unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurationSeconds")]
    pub max_duration_seconds: Option<String>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<String>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A video's URL. The server must be publicly routable and support `HTTP HEAD`
    /// requests and `HTTP GET` range requests. The server should respond to `HTTP HEAD`
    /// requests with a `content-range` header that includes the size of the file.
    pub url: String,

    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<String>,

    /// Indicates the date and time at which the video will be deleted. Omit the field
    /// to indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion. If specified, must be at least 30 days from upload time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<String>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_creator: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectUploadNewResponse {
    /// Indicates the date and time at which the video will be deleted. Omit the field
    /// to indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion. If specified, must be at least 30 days from upload time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<DateTime<Utc>>,

    /// A Cloudflare-generated unique identifier for a media item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The URL an unauthenticated upload can use for a single
    /// `HTTP POST multipart/form-data` request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uploadURL")]
    pub upload_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<Watermark>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectUploadNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The maximum duration in seconds for a video upload. Can be set for a video that
    /// is not yet uploaded to limit its duration. Uploads that exceed the specified
    /// duration will fail during processing. A value of `-1` means the value is
    /// unknown.
    #[serde(rename = "maxDurationSeconds")]
    pub max_duration_seconds: String,

    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// The date and time after upload when videos will not be accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<String>,

    /// Indicates the date and time at which the video will be deleted. Omit the field
    /// to indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion. If specified, must be at least 30 days from upload time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<String>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_creator: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadNewResponse {
    /// Indicates the progress as a percentage between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "percentComplete")]
    pub percent_complete: Option<f64>,

    /// The status of a generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DownloadNewResponseStatus>,

    /// The URL to access the generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadGetResponse {
    /// The audio-only download. Only present if this download type has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<DownloadGetResponseAudio>,

    /// The default video download. Only present if this download type has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DownloadGetResponseDefault>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The date and time a signing key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The signing key in JWK format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwk: Option<String>,

    /// The signing key in PEM format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGetResponse {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The date and time a signing key was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInput {
    /// The date and time the live input was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Indicates the number of days after which the live inputs recordings will be
    /// deleted. When a stream completes and the recording is ready, the value is used
    /// to calculate a scheduled deletion date for that recording. Omit the field to
    /// indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteRecordingAfterDays")]
    pub delete_recording_after_days: Option<f64>,

    /// Indicates whether the live input is enabled and can accept streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing live inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// The date and time the live input was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// Records the input to a Cloudflare Stream video. Behavior depends on the mode. In
    /// most cases, the video will initially be viewable as a live video and transition
    /// to on-demand after a condition is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<LiveInputRecording>,

    /// Details for streaming to an live input using RTMPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmps: Option<LiveInputRtmps>,

    /// Details for playback from an live input using RTMPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rtmpsPlayback")]
    pub rtmps_playback: Option<LiveInputRtmpsPlayback>,

    /// Details for streaming to a live input using SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<LiveInputSrt>,

    /// Details for playback from an live input using SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "srtPlayback")]
    pub srt_playback: Option<LiveInputSrtPlayback>,

    /// The connection status of a live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LiveInputStatus>,

    /// A unique identifier for a live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Details for streaming to a live input using WebRTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "webRTC")]
    pub web_rtc: Option<LiveInputWebRtc>,

    /// Details for playback from a live input using WebRTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "webRTCPlayback")]
    pub web_rtc_playback: Option<LiveInputWebRtcPlayback>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "liveInputs")]
    pub live_inputs: Option<Vec<LiveInputListResponseLiveInput>>,

    /// The total number of remaining live inputs based on cursor position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,

    /// The total number of live inputs that match the provided filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Sets the creator ID asssociated with this live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultCreator")]
    pub default_creator: Option<String>,

    /// Indicates the number of days after which the live inputs recordings will be
    /// deleted. When a stream completes and the recording is ready, the value is used
    /// to calculate a scheduled deletion date for that recording. Omit the field to
    /// indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteRecordingAfterDays")]
    pub delete_recording_after_days: Option<String>,

    /// Indicates whether the live input is enabled and can accept streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing live inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Records the input to a Cloudflare Stream video. Behavior depends on the mode. In
    /// most cases, the video will initially be viewable as a live video and transition
    /// to on-demand after a condition is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Sets the creator ID asssociated with this live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultCreator")]
    pub default_creator: Option<String>,

    /// Indicates the number of days after which the live inputs recordings will be
    /// deleted. When a stream completes and the recording is ready, the value is used
    /// to calculate a scheduled deletion date for that recording. Omit the field to
    /// indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteRecordingAfterDays")]
    pub delete_recording_after_days: Option<String>,

    /// Indicates whether the live input is enabled and can accept streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing live inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Records the input to a Cloudflare Stream video. Behavior depends on the mode. In
    /// most cases, the video will initially be viewable as a live video and transition
    /// to on-demand after a condition is satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Includes the total number of videos associated with the submitted query
    /// parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_counts: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputGetParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    /// When enabled, live video streamed to the associated live input will be sent to
    /// the output URL. When disabled, live video will not be sent to the output URL,
    /// even when streaming to the associated live input. Use this to control precisely
    /// when you start and stop simulcasting to specific destinations like YouTube and
    /// Twitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The streamKey used to authenticate against an output's target.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamKey")]
    pub stream_key: Option<String>,

    /// A unique identifier for the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The URL an output uses to restream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputOutputNewParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The streamKey used to authenticate against an output's target.
    #[serde(rename = "streamKey")]
    pub stream_key: String,

    /// The URL an output uses to restream.
    pub url: String,

    /// When enabled, live video streamed to the associated live input will be sent to
    /// the output URL. When disabled, live video will not be sent to the output URL,
    /// even when streaming to the associated live input. Use this to control precisely
    /// when you start and stop simulcasting to specific destinations like YouTube and
    /// Twitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputOutputUpdateParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// When enabled, live video streamed to the associated live input will be sent to
    /// the output URL. When disabled, live video will not be sent to the output URL,
    /// even when streaming to the associated live input. Use this to control precisely
    /// when you start and stop simulcasting to specific destinations like YouTube and
    /// Twitch.
    pub enabled: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputOutputListParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputOutputDeleteParams {
    /// Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<Vec<String>>,

    /// The date and time the media item was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// The duration of the video in seconds. A value of `-1` means the duration is
    /// unknown. The duration becomes available after the upload and before the video is
    /// ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<VideoInput>,

    /// The live input ID used to upload a video with Stream Live.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "liveInput")]
    pub live_input: Option<String>,

    /// The maximum duration in seconds for a video upload. Can be set for a video that
    /// is not yet uploaded to limit its duration. Uploads that exceed the specified
    /// duration will fail during processing. A value of `-1` means the value is
    /// unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurationSeconds")]
    pub max_duration_seconds: Option<i64>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// The date and time the media item was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback: Option<VideoPlayback>,

    /// The video's preview page URI. This field is omitted until encoding is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    /// Indicates whether the video is playable. The field is empty if the video is not
    /// ready for viewing or the live stream is still in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readyToStream")]
    pub ready_to_stream: Option<bool>,

    /// Indicates the time at which the video became playable. The field is empty if the
    /// video is not ready for viewing or the live stream is still in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readyToStreamAt")]
    pub ready_to_stream_at: Option<DateTime<Utc>>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<bool>,

    /// Indicates the date and time at which the video will be deleted. Omit the field
    /// to indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion. If specified, must be at least 30 days from upload time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<DateTime<Utc>>,

    /// The size of the media item in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    /// Specifies a detailed status for a video. If the `state` is `inprogress` or
    /// `error`, the `step` field returns `encoding` or `manifest`. If the `state` is
    /// `inprogress`, `pctComplete` returns a number between 0 and 100 to indicate the
    /// approximate percent of completion. If the `state` is `error`, `errorReasonCode`
    /// and `errorReasonText` provide additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VideoStatus>,

    /// The media item's thumbnail URI. This field is omitted until encoding is
    /// complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<f64>,

    /// A Cloudflare-generated unique identifier for a media item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The date and time the media item was uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<DateTime<Utc>>,

    /// The date and time when the video upload URL is no longer valid for direct user
    /// uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uploadExpiry")]
    pub upload_expiry: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<Watermark>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    pub body: String,

    /// Specifies the TUS protocol version. This value must be included in every upload
    /// request. Notes: The only supported version of TUS protocol is 1.0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tus_resumable: Option<String>,

    /// Indicates the size of the entire upload in bytes. The value must be a
    /// non-negative integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_length: Option<String>,

    /// Provisions a URL to let your end users upload videos directly to Cloudflare
    /// Stream without exposing your API token to clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_user: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_creator: Option<String>,

    /// Comma-separated key-value pairs following the TUS protocol specification. Values
    /// are Base-64 encoded. Supported keys: `name`, `requiresignedurls`,
    /// `allowedorigins`, `thumbnailtimestamppct`, `watermark`, `scheduleddeletion`,
    /// `maxdurationseconds`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_metadata: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamListParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Lists videos in ascending order of creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asc: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// Lists videos created before the specified date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Includes the total number of videos associated with the submitted query
    /// parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_counts: Option<String>,

    /// Provides a partial word match of the `name` key in the `meta` field. Slow for
    /// medium to large video libraries. May be unavailable for very large libraries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Lists videos created after the specified date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// Specifies the processing status for all quality levels for a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Specifies whether the video is `vod` or `live`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Provides a fast, exact string match on the `name` key in the `meta` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_name: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEditParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// Lists the origins allowed to display the video. Enter allowed origin domains in
    /// an array and use `*` for wildcard subdomains. Empty arrays allow the video to be
    /// viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// The maximum duration in seconds for a video upload. Can be set for a video that
    /// is not yet uploaded to limit its duration. Uploads that exceed the specified
    /// duration will fail during processing. A value of `-1` means the value is
    /// unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxDurationSeconds")]
    pub max_duration_seconds: Option<String>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Indicates whether the video can be a accessed using the UID. When set to `true`,
    /// a signed token must be generated with a signing key to view the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<String>,

    /// Indicates the date and time at which the video will be deleted. Omit the field
    /// to indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion. If specified, must be at least 30 days from upload time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scheduledDeletion")]
    pub scheduled_deletion: Option<String>,

    /// The timestamp for a thumbnail image calculated as a percentage value of the
    /// video's duration. To convert from a second-wise timestamp to a percentage,
    /// divide the desired timestamp by the total duration of the video. If this value
    /// is not set, the default thumbnail image is taken from 0s of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thumbnailTimestampPct")]
    pub thumbnail_timestamp_pct: Option<String>,

    /// The date and time when the video upload URL is no longer valid for direct user
    /// uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uploadExpiry")]
    pub upload_expiry: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewResponse {
    /// The signed token used with the signed URLs feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The optional ID of a Stream signing key. If present, the `pem` field is also
    /// required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The optional list of access rule constraints on the token. Access can be blocked
    /// or allowed based on an IP, IP range, or by country. Access rules are evaluated
    /// from first to last. If a rule matches, the associated action is applied and no
    /// further rules are evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accessRules")]
    pub access_rules: Option<String>,

    /// The optional boolean value that enables using signed tokens to access MP4
    /// download links for a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloadable: Option<String>,

    /// The optional unix epoch timestamp that specficies the time after a token is not
    /// accepted. The maximum time specification is 24 hours from issuing time. If this
    /// field is not set, the default is one hour after issuing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<String>,

    /// The optional unix epoch timestamp that specifies the time before a the token is
    /// not accepted. If this field is not set, the default is one hour before issuing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<String>,

    /// The optional base64 encoded private key in PEM format associated with a Stream
    /// signing key. If present, the `id` field is also required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStorageUsageResponse {
    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// The total minutes of video content stored in the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalStorageMinutes")]
    pub total_storage_minutes: Option<i64>,

    /// The storage capacity alloted for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalStorageMinutesLimit")]
    pub total_storage_minutes_limit: Option<i64>,

    /// The total count of videos associated with the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "videoCount")]
    pub video_count: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStorageUsageParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// A user-defined identifier for the media creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watermark {
    /// The date and a time a watermark profile was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// The source URL for a downloaded image. If the watermark profile was created via
    /// direct upload, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downloadedFrom")]
    pub downloaded_from: Option<String>,

    /// The height of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// A short description of the watermark profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The translucency of the image. A value of `0.0` makes the image completely
    /// transparent, and `1.0` makes the image completely opaque. Note that if the image
    /// is already semi-transparent, setting this to `1.0` will not make the image
    /// completely opaque.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,

    /// The whitespace between the adjacent edges (determined by position) of the video
    /// and the image. `0.0` indicates no padding, and `1.0` indicates a fully padded
    /// video width or length, as determined by the algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,

    /// The location of the image. Valid positions are: `upperRight`, `upperLeft`,
    /// `lowerLeft`, `lowerRight`, and `center`. Note that `center` ignores the
    /// `padding` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    /// The size of the image relative to the overall size of the video. This parameter
    /// will adapt to horizontal and vertical videos automatically. `0.0` indicates no
    /// scaling (use the size of the image as-is), and `1.0 `fills the entire video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,

    /// The size of the image in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,

    /// The unique identifier for a watermark profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The width of the image in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkNewParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The image file to upload.
    pub file: String,

    /// A short description of the watermark profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The translucency of the image. A value of `0.0` makes the image completely
    /// transparent, and `1.0` makes the image completely opaque. Note that if the image
    /// is already semi-transparent, setting this to `1.0` will not make the image
    /// completely opaque.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<String>,

    /// The whitespace between the adjacent edges (determined by position) of the video
    /// and the image. `0.0` indicates no padding, and `1.0` indicates a fully padded
    /// video width or length, as determined by the algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,

    /// The location of the image. Valid positions are: `upperRight`, `upperLeft`,
    /// `lowerLeft`, `lowerRight`, and `center`. Note that `center` ignores the
    /// `padding` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    /// The size of the image relative to the overall size of the video. This parameter
    /// will adapt to horizontal and vertical videos automatically. `0.0` indicates no
    /// scaling (use the size of the image as-is), and `1.0 `fills the entire video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkListParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookUpdateParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

    /// The URL where webhooks will be sent.
    #[serde(rename = "notificationUrl")]
    pub notification_url: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDeleteParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookGetParams {
    /// The account identifier tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipPlayback {
    /// DASH Media Presentation Description for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash: Option<String>,

    /// The HLS manifest for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipWatermark {
    /// The unique identifier for the watermark profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadGetResponseAudio {
    /// Indicates the progress as a percentage between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "percentComplete")]
    pub percent_complete: Option<f64>,

    /// The status of a generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DownloadGetResponseAudioStatus>,

    /// The URL to access the generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadGetResponseDefault {
    /// Indicates the progress as a percentage between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "percentComplete")]
    pub percent_complete: Option<f64>,

    /// The status of a generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DownloadGetResponseDefaultStatus>,

    /// The URL to access the generated download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputRecording {
    /// Lists the origins allowed to display videos created with this input. Enter
    /// allowed origin domains in an array and use `*` for wildcard subdomains. An empty
    /// array allows videos to be viewed on any origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: Option<Vec<String>>,

    /// Disables reporting the number of live viewers when this property is set to
    /// `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideLiveViewerCount")]
    pub hide_live_viewer_count: Option<bool>,

    /// Specifies the recording behavior for the live input. Set this value to `off` to
    /// prevent a recording. Set the value to `automatic` to begin a recording and
    /// transition to on-demand after Stream Live stops receiving input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<LiveInputRecordingMode>,

    /// Indicates if a video using the live input has the `requireSignedURLs` property
    /// set. Also enforces access controls on any video recording of the livestream with
    /// the live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<bool>,

    /// Determines the amount of time a live input configured in `automatic` mode should
    /// wait before a recording transitions from live to on-demand. `0` is recommended
    /// for most use cases and indicates the platform default should be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputRtmps {
    /// The secret key to use when streaming via RTMPS to a live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamKey")]
    pub stream_key: Option<String>,

    /// The RTMPS URL you provide to the broadcaster, which they stream live video to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputRtmpsPlayback {
    /// The secret key to use for playback via RTMPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamKey")]
    pub stream_key: Option<String>,

    /// The URL used to play live video over RTMPS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputSrt {
    /// The secret key to use when streaming via SRT to a live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,

    /// The identifier of the live input to use when streaming via SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamId")]
    pub stream_id: Option<String>,

    /// The SRT URL you provide to the broadcaster, which they stream live video to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputSrtPlayback {
    /// The secret key to use for playback via SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,

    /// The identifier of the live input to use for playback via SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "streamId")]
    pub stream_id: Option<String>,

    /// The URL used to play live video over SRT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputWebRtc {
    /// The WebRTC URL you provide to the broadcaster, which they stream live video to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputWebRtcPlayback {
    /// The URL used to play live video over WebRTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInputListResponseLiveInput {
    /// The date and time the live input was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Indicates the number of days after which the live inputs recordings will be
    /// deleted. When a stream completes and the recording is ready, the value is used
    /// to calculate a scheduled deletion date for that recording. Omit the field to
    /// indicate no change, or include with a `null` value to remove an existing
    /// scheduled deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deleteRecordingAfterDays")]
    pub delete_recording_after_days: Option<f64>,

    /// Indicates whether the live input is enabled and can accept streams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// A user modifiable key-value store used to reference other systems of record for
    /// managing live inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// The date and time the live input was last modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    /// A unique identifier for a live input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInput {
    /// The video height in pixels. A value of `-1` means the height is unknown. The
    /// value becomes available after the upload and before the video is ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// The video width in pixels. A value of `-1` means the width is unknown. The value
    /// becomes available after the upload and before the video is ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPlayback {
    /// DASH Media Presentation Description for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash: Option<String>,

    /// The HLS manifest for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStatus {
    /// Specifies why the video failed to encode. This field is empty if the video is
    /// not in an `error` state. Preferred for programmatic use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorReasonCode")]
    pub error_reason_code: Option<String>,

    /// Specifies why the video failed to encode using a human readable error message in
    /// English. This field is empty if the video is not in an `error` state.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errorReasonText")]
    pub error_reason_text: Option<String>,

    /// Indicates the progress as a percentage between 0 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pctComplete")]
    pub pct_complete: Option<String>,

    /// Specifies the processing status for all quality levels for a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<VideoStatusState>,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioStatus {
    #[serde(rename = "queued")]
    AudioStatusQueued,
    #[serde(rename = "ready")]
    AudioStatusReady,
    #[serde(rename = "error")]
    AudioStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CaptionStatus {
    #[serde(rename = "ready")]
    CaptionStatusReady,
    #[serde(rename = "inprogress")]
    CaptionStatusInprogress,
    #[serde(rename = "error")]
    CaptionStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClipStatus {
    #[serde(rename = "pendingupload")]
    ClipStatusPendingupload,
    #[serde(rename = "downloading")]
    ClipStatusDownloading,
    #[serde(rename = "queued")]
    ClipStatusQueued,
    #[serde(rename = "inprogress")]
    ClipStatusInprogress,
    #[serde(rename = "ready")]
    ClipStatusReady,
    #[serde(rename = "error")]
    ClipStatusError,
    #[serde(rename = "live-inprogress")]
    ClipStatusLiveInprogress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadNewResponseStatus {
    #[serde(rename = "ready")]
    DownloadNewResponseStatusReady,
    #[serde(rename = "inprogress")]
    DownloadNewResponseStatusInprogress,
    #[serde(rename = "error")]
    DownloadNewResponseStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiveInputStatus {
    #[serde(rename = "connected")]
    LiveInputStatusConnected,
    #[serde(rename = "reconnected")]
    LiveInputStatusReconnected,
    #[serde(rename = "reconnecting")]
    LiveInputStatusReconnecting,
    #[serde(rename = "client_disconnect")]
    LiveInputStatusClientDisconnect,
    #[serde(rename = "ttl_exceeded")]
    LiveInputStatusTTLExceeded,
    #[serde(rename = "failed_to_connect")]
    LiveInputStatusFailedToConnect,
    #[serde(rename = "failed_to_reconnect")]
    LiveInputStatusFailedToReconnect,
    #[serde(rename = "new_configuration_accepted")]
    LiveInputStatusNewConfigurationAccepted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadGetResponseAudioStatus {
    #[serde(rename = "ready")]
    DownloadGetResponseAudioStatusReady,
    #[serde(rename = "inprogress")]
    DownloadGetResponseAudioStatusInprogress,
    #[serde(rename = "error")]
    DownloadGetResponseAudioStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadGetResponseDefaultStatus {
    #[serde(rename = "ready")]
    DownloadGetResponseDefaultStatusReady,
    #[serde(rename = "inprogress")]
    DownloadGetResponseDefaultStatusInprogress,
    #[serde(rename = "error")]
    DownloadGetResponseDefaultStatusError,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LiveInputRecordingMode {
    #[serde(rename = "off")]
    LiveInputRecordingModeOff,
    #[serde(rename = "automatic")]
    LiveInputRecordingModeAutomatic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoStatusState {
    #[serde(rename = "pendingupload")]
    VideoStatusStatePendingupload,
    #[serde(rename = "downloading")]
    VideoStatusStateDownloading,
    #[serde(rename = "queued")]
    VideoStatusStateQueued,
    #[serde(rename = "inprogress")]
    VideoStatusStateInprogress,
    #[serde(rename = "ready")]
    VideoStatusStateReady,
    #[serde(rename = "error")]
    VideoStatusStateError,
    #[serde(rename = "live-inprogress")]
    VideoStatusStateLiveInprogress,
}

