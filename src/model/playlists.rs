use super::{
    ExternalUrls, Followers, Image, ItemType, Page, TrackItem, TrackReference, UserReference,
    VideoThumbnail,
};
use serde::{Deserialize, Serialize};

/// Full playlist information from the Spotify catalog.
///
/// Contains complete details about a playlist including its tracks (when the
/// `page_items` feature is enabled), description, and follower count.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Playlist {
    /// true if the owner allows other users to modify the playlist.
    pub collaborative: bool,

    /// The playlist description. Only returned for modified, verified playlists, otherwise None.
    pub description: Option<String>,

    /// Known external URLs for this playlist.
    pub external_urls: ExternalUrls,

    /// Information about the followers of the playlist.
    pub followers: Followers,

    /// A link to the Web API endpoint providing full details of the playlist.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
    pub id: String,

    /// Images for the playlist. The array may be empty or contain up to three images.
    pub images: Option<Vec<Image>>,

    /// The name of the playlist.
    pub name: String,

    /// The user who owns the playlist
    pub owner: UserReference,

    /// Gradient color of the playlist's cover image.
    /// Will always be None as this is only used internally by Spotify.
    pub primary_color: Option<String>,

    /// The playlist's public/private status: true the playlist is public, false the playlist is private,
    /// null the playlist status is not relevant.
    pub public: Option<bool>,

    /// The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
    pub snapshot_id: String,

    /// The tracks of the playlist.
    #[cfg(feature = "page_items")]
    pub tracks: Page<PlaylistTrack>,

    /// The object type: "playlist"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
    pub uri: String,
}

/// Simplified playlist information with basic details only.
///
/// A lighter version of [`Playlist`] that omits follower count and includes
/// only a track reference instead of the full track list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylist {
    /// true if the owner allows other users to modify the playlist.
    pub collaborative: bool,

    /// The playlist description. Only returned for modified, verified playlists, otherwise None.
    pub description: Option<String>,

    /// Known external URLs for this playlist.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the playlist.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
    pub id: String,

    /// Images for the playlist. The array may be empty or contain up to three images.
    pub images: Option<Vec<Image>>,

    /// The name of the playlist.
    pub name: String,

    /// The user who owns the playlist
    pub owner: UserReference,

    /// Gradient color of the playlist's cover image.
    /// Will always be None as this is only used internally by Spotify.
    pub primary_color: Option<String>,

    /// The playlist's public/private status: true the playlist is public, false the playlist is private,
    /// null the playlist status is not relevant.
    pub public: Option<bool>,

    /// The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
    pub snapshot_id: String,

    /// A collection containing a link ( href ) to the Web API endpoint where full details of the playlist's tracks can be retrieved,
    /// along with the total number of tracks in the playlist. Note, a track object may be null. This can happen if a track is no longer available.
    pub tracks: Option<TrackReference>,

    /// The object type: "playlist"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the playlist.
    pub uri: String,
}

impl From<Playlist> for SimplifiedPlaylist {
    fn from(playlist: Playlist) -> Self {
        let tracks = {
            #[cfg(feature = "page_items")]
            {
                Some(TrackReference {
                    href: playlist.tracks.href,
                    total: playlist.tracks.total,
                })
            }
            #[cfg(not(feature = "page_items"))]
            {
                None
            }
        };

        Self {
            collaborative: playlist.collaborative,
            description: playlist.description,
            external_urls: playlist.external_urls,
            href: playlist.href,
            id: playlist.id,
            images: playlist.images,
            name: playlist.name,
            owner: playlist.owner,
            primary_color: playlist.primary_color,
            public: playlist.public,
            snapshot_id: playlist.snapshot_id,
            tracks,
            type_: playlist.type_,
            uri: playlist.uri,
        }
    }
}

/// A track or episode within a playlist.
///
/// Contains information about when the item was added, who added it,
/// whether it's a local file, and the track/episode details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistTrack {
    /// The date and time the track or episode was added.
    /// Note: some very old playlists may return None in this field.
    pub added_at: Option<String>,

    /// The Spotify user who added the track or episode.
    ///
    /// # Note
    /// Some very old playlists may return None in this field.
    pub added_by: Option<AddedBy>,

    /// Whether this track or episode is a [local file](https://developer.spotify.com/documentation/web-api/concepts/playlists#local-files) or not.
    pub is_local: bool,

    /// Gradient color of the playlist's cover image.
    /// Will always be None as this is only used internally by Spotify.
    pub primary_color: Option<String>,

    pub video_thumbnail: Option<VideoThumbnail>,

    /// Information about the track or episode.
    pub track: TrackItem,
}

/// Information about the user who added a track or episode to a playlist.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddedBy {
    /// Known public external URLs for this user.
    pub external_urls: ExternalUrls,

    /// Information about the followers of this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Followers>,

    /// A link to the Web API endpoint for this user.
    pub href: String,

    /// The [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub id: String,

    /// The object type.
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub uri: String,
}

/// A playlist snapshot identifier.
///
/// Returned after modifying a playlist to identify the specific version.
/// Can be used in subsequent requests to target a specific playlist state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotId {
    /// The version identifier for the current playlist.
    /// Can be supplied in other requests to target a specific playlist version.
    pub snapshot_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playlist() {
        let json = r#"
        {
			"collaborative": false,
			"description": "string",
			"external_urls": {
				"spotify": "string"
			},
			"followers": {
				"href": "string",
				"total": 0
			},
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
			"owner": {
				"external_urls": {
					"spotify": "string"
				},
				"followers": {
					"href": "string",
					"total": 0
				},
				"href": "string",
				"id": "string",
				"type": "user",
				"uri": "string",
				"display_name": "string"
			},
			"primary_color": null,
			"public": false,
			"snapshot_id": "string",
			"tracks": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
						"added_at": "string",
						"added_by": {
							"external_urls": {
								"spotify": "string"
							},
							"followers": {
								"href": "string",
								"total": 0
							},
							"href": "string",
							"id": "string",
							"type": "user",
							"uri": "string"
						},
						"is_local": false,
						"primary_color": null,
						"video_thumbnail": {
							"url": null
						},
						"track": {
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
						}
					}
				]
			},
			"type": "playlist",
			"uri": "string"
        }
        "#;

        crate::test::assert_deserialized!(Playlist, json);
    }

    #[test]
    fn simplified_playlist() {
        let json = r#"
        {
			"collaborative": false,
			"description": "string",
			"external_urls": {
				"spotify": "string"
			},
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
			"owner": {
				"external_urls": {
					"spotify": "string"
				},
				"followers": {
					"href": "string",
					"total": 0
				},
				"href": "string",
				"id": "string",
				"type": "user",
				"uri": "string",
				"display_name": "string"
			},
			"primary_color": null,
			"public": false,
			"snapshot_id": "string",
			"tracks": {
				"href": "string",
				"total": 0
			},
			"type": "playlist",
			"uri": "string"
        }
        "#;

        crate::test::assert_deserialized!(SimplifiedPlaylist, json);
    }
}
