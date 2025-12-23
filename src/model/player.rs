use super::{ContextType, Cursors, EpisodeId, ExternalUrls, ItemType, Track, TrackId, TrackItem};
use serde::{Deserialize, Serialize};

/// A playback device (speaker, phone, computer, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    /// The device ID. This ID is unique and persistent to some extent.
    /// However, this is not guaranteed and any cached `device_id` should periodically be cleared out and refetched as necessary.
    pub id: Option<String>,

    /// If this device is the currently active device.
    pub is_active: bool,

    /// If this device is currently in a private session.
    pub is_private_session: bool,

    /// Whether controlling this device is restricted.
    /// At present if this is "true" then no Web API commands will be accepted by this device.
    pub is_restricted: bool,

    /// A human-readable name for the device. Some devices have a name that the user can configure (e.g. "Loudest speaker")
    /// and some devices have a generic name associated with the manufacturer or device model.
    pub name: String,

    #[serde(rename = "type")]
    /// Device type, such as "computer", "smartphone" or "speaker".
    pub type_: String,

    /// The current volume in percent.
    pub volume_percent: Option<u8>,

    /// If this device can be used to set the volume.
    pub supports_volume: bool,
}

/// A list of available playback devices.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Devices {
    /// The list of devices.
    #[serde(default)]
    pub devices: Vec<Device>,
}

/// The repeat mode state for playback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RepeatState {
    Track,
    Context,
    Off,
}

impl std::fmt::Display for RepeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Track => "track",
            Self::Context => "context",
            Self::Off => "off",
        };
        write!(f, "{s}")
    }
}

/// The playback context (album, playlist, artist, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    /// The object type.
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// A link to the Web API endpoint providing full details of the track.
    pub href: Option<String>,

    /// External URLs for this context.
    pub external_urls: ExternalUrls,

    /// The Spotify URI for the context.
    pub uri: String,
}

/// The type of the currently playing item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Ad,
    Unknown,
}

/// The current playback state including device, track, and progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaybackState {
    /// The device that is currently active.
    pub device: Device,

    /// off, track, context
    pub repeat_state: RepeatState,

    /// If shuffle is on or off.
    pub shuffle_state: bool,

    /// The context object.
    pub context: Option<Context>,

    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: Option<i64>,

    /// Progress into the currently playing track or episode.
    pub progress_ms: Option<u32>,

    /// If something is currently playing, return true.
    pub is_playing: bool,

    /// The currently playing track or episode.
    pub item: Option<TrackItem>,

    /// The object type of the currently playing item. Can be one of track, episode, ad or unknown.
    pub currently_playing_type: CurrentlyPlayingType,

    /// Allows to update the user interface based on which playback actions are available within the current context.
    pub actions: Actions,
}

/// Available playback actions in the current context.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Actions {
    /// Interrupting playback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupting_playback: Option<bool>,

    /// Pausing playback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pausing: Option<bool>,

    /// Resuming playback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resuming: Option<bool>,

    /// Seeking playback location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seeking: Option<bool>,

    /// Skipping to the next context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipping_next: Option<bool>,

    /// Skipping to the previous context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipping_prev: Option<bool>,

    /// Toggling repeat context flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_context: Option<bool>,

    /// Toggling shuffle flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_shuffle: Option<bool>,

    /// Toggling repeat track flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_track: Option<bool>,

    /// Transferring playback between devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferring_playback: Option<bool>,
}

/// Information about the currently playing track or episode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlaying {
    /// The context object.
    pub context: Option<Context>,

    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: Option<i64>,

    /// Progress into the currently playing track or episode.
    pub progress_ms: Option<u32>,

    /// If something is currently playing, return true.
    pub is_playing: bool,

    /// The currently playing track or episode.
    pub item: Option<TrackItem>,

    /// The object type of the currently playing item. Can be one of track, episode, ad or unknown.
    pub currently_playing_type: CurrentlyPlayingType,

    /// Allows to update the user interface based on which playback actions are available within the current context.
    pub actions: Actions,
}

/// A track in the user's play history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayHistory {
    /// The track the user listened to.
    pub track: Track,

    /// The date and time the track was played.
    pub played_at: String,

    /// The context the track was played from.
    pub context: Context,
}

/// A list of recently played tracks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentlyPlayedTracks {
    /// A link to the Web API endpoint returning the full result of the request.
    pub href: String,

    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: usize,

    /// URL to the next page of items.
    pub next: Option<String>,

    /// The cursors used to find the next set of items.
    pub cursors: Option<Cursors>,

    /// The total number of items available to return.
    pub total: Option<usize>,

    /// The play history items.
    pub items: Vec<PlayHistory>,
}

/// The user's playback queue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Queue {
    /// The currently playing track or episode.
    pub currently_playing: Option<TrackItem>,

    /// The tracks or episodes in the queue.
    pub queue: Vec<TrackItem>,
}

/// An offset for starting playback at a specific position or URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    Position(usize),
    Uri(ContextType),
}

impl From<usize> for Offset {
    fn from(position: usize) -> Self {
        Self::Position(position)
    }
}

/// A time range for querying recently played tracks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryRange {
    Before(i64),
    After(i64),
}

/// An item that can be added to a playlist (track or episode).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaylistItem {
    Track(TrackId),
    Episode(EpisodeId),
}

impl From<TrackId> for PlaylistItem {
    fn from(track: TrackId) -> Self {
        Self::Track(track)
    }
}

impl From<EpisodeId> for PlaylistItem {
    fn from(episode: EpisodeId) -> Self {
        Self::Episode(episode)
    }
}

impl std::fmt::Display for PlaylistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Track(track) => track.uri(),
            Self::Episode(episode) => episode.uri(),
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_state() {
        let json = r#"
        {
			"device": {
				"id": "string",
				"is_active": false,
				"is_private_session": false,
				"is_restricted": false,
				"name": "Kitchen speaker",
				"type": "computer",
				"volume_percent": 59,
				"supports_volume": false
			},
			"repeat_state": "off",
			"shuffle_state": false,
			"context": {
				"type": "track",
				"href": "string",
				"external_urls": {
					"spotify": "string"
				},
				"uri": "string"
			},
			"timestamp": 0,
			"progress_ms": 0,
			"is_playing": false,
			"item": {
				"album": {
					"album_type": "compilation",
					"total_tracks": 9,
					"available_markets": ["CA", "BR", "IT"],
					"external_urls": {
						"spotify": "string"
					},
					"href": "string",
					"id": "2up3OPMp9Tb4dAKM2erWXQ",
					"images": [
						{
							"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
							"height": 300,
							"width": 300
						}
					],
					"name": "string",
					"release_date": "1981-12",
					"release_date_precision": "year",
					"restrictions": {
						"reason": "market"
					},
					"type": "album",
					"uri": "spotify:album:2up3OPMp9Tb4dAKM2erWXQ",
					"artists": [
						{
							"external_urls": {
								"spotify": "string"
							},
							"href": "string",
							"id": "string",
							"name": "string",
							"type": "artist",
							"uri": "string"
						}
					]
				},
				"artists": [
					{
						"external_urls": {
							"spotify": "string"
						},
						"followers": {
							"href": "string",
							"total": 0
						},
						"genres": ["Prog rock", "Grunge"],
						"href": "string",
						"id": "string",
						"images": [
							{
								"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
								"height": 300,
								"width": 300
							}
						],
						"name": "string",
						"popularity": 0,
						"type": "artist",
						"uri": "string"
					}
				],
				"available_markets": ["US"],
				"disc_number": 0,
				"duration_ms": 0,
				"explicit": false,
				"external_ids": {
					"isrc": "string",
					"ean": "string",
					"upc": "string"
				},
				"external_urls": {
					"spotify": "string"
				},
				"href": "string",
				"id": "string",
				"is_playable": false,
				"linked_from": {},
				"restrictions": {
					"reason": "string"
				},
				"name": "string",
				"popularity": 0,
				"preview_url": "string",
				"track_number": 0,
				"type": "track",
				"uri": "string",
				"is_local": false
			},
			"currently_playing_type": "unknown",
			"actions": {
				"interrupting_playback": false,
				"pausing": false,
				"resuming": false,
				"seeking": false,
				"skipping_next": false,
				"skipping_prev": false,
				"toggling_repeat_context": false,
				"toggling_shuffle": false,
				"toggling_repeat_track": false,
				"transferring_playback": false
			}
        }
        "#;

        crate::test::assert_deserialized!(PlaybackState, json);
    }
}
