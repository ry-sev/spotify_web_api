use super::{
    Artist, Page, SimplifiedAlbum, SimplifiedAudiobook, SimplifiedEpisode, SimplifiedPlaylist,
    SimplifiedShow, Track,
};
use serde::{Deserialize, Serialize};

/// Spotify catalog information about albums, artists, playlists, tracks, shows, episodes or audiobooks.
///
/// Contains paginated results for each type of item that was searched for.
/// Only the types requested in the search will have values; others will be `None`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResults {
    /// Matching playlists, if playlists were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub playlists: Option<Page<Option<SimplifiedPlaylist>>>,

    /// Matching albums, if albums were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub albums: Option<Page<Option<SimplifiedAlbum>>>,

    /// Matching artists, if artists were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub artists: Option<Page<Option<Artist>>>,

    /// Matching tracks, if tracks were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tracks: Option<Page<Option<Track>>>,

    /// Matching shows (podcasts), if shows were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shows: Option<Page<Option<SimplifiedShow>>>,

    /// Matching episodes, if episodes were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub episodes: Option<Page<Option<SimplifiedEpisode>>>,

    /// Matching audiobooks, if audiobooks were included in the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audiobooks: Option<Page<Option<SimplifiedAudiobook>>>,
}

/// The type of item to search for in the Spotify catalog.
///
/// Used with the search endpoint to specify which types of items to include
/// in the search results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SearchType {
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
    Audiobook,
}

impl SearchType {
    pub fn all() -> &'static [Self] {
        &[
            Self::Album,
            Self::Artist,
            Self::Playlist,
            Self::Track,
            Self::Show,
            Self::Episode,
            Self::Audiobook,
        ]
    }
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Show => "show",
            Self::Episode => "episode",
            Self::Audiobook => "audiobook",
        };

        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let json = r#"
        {
			"tracks": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
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
								"href": "string",
								"id": "string",
								"name": "string",
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
				]
			},
			"artists": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
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
				]
			},
			"albums": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
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
					}
				]
			},
			"playlists": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
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
						"public": false,
						"snapshot_id": "string",
						"tracks": {
							"href": "string",
							"total": 0
						},
						"type": "playlist",
						"uri": "string"
					}
				]
			},
			"shows": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
						"available_markets": ["US"],
						"copyrights": [
							{
								"text": "string",
								"type": "C"
							}
						],
						"description": "string",
						"html_description": "string",
						"explicit": false,
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
						"is_externally_hosted": false,
						"languages": ["string"],
						"media_type": "string",
						"name": "string",
						"publisher": "string",
						"type": "show",
						"uri": "string",
						"total_episodes": 0
					}
				]
			},
			"episodes": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
						"audio_preview_url": "https://p.scdn.co/mp3-preview/2f37da1d4221f40b9d1a98cd191f4d6f1646ad17",
						"description": "A Spotify podcast sharing fresh insights on important topics of the moment—in a way only Spotify can. You’ll hear from experts in the music, podcast and tech industries as we discover and uncover stories about our work and the world around us.",
						"html_description": "<p>A Spotify podcast sharing fresh insights on important topics of the moment—in a way only Spotify can. You’ll hear from experts in the music, podcast and tech industries as we discover and uncover stories about our work and the world around us.</p>",
						"duration_ms": 1686230,
						"explicit": false,
						"external_urls": {
							"spotify": "string"
						},
						"href": "https://api.spotify.com/v1/episodes/5Xt5DXGzch68nYYamXrNxZ",
						"id": "5Xt5DXGzch68nYYamXrNxZ",
						"images": [
							{
								"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
								"height": 300,
								"width": 300
							}
						],
						"is_externally_hosted": false,
						"is_playable": false,
						"language": "en",
						"languages": ["fr", "en"],
						"name": "Starting Your Own Podcast: Tips, Tricks, and Advice From Anchor Creators",
						"release_date": "1981-12-15",
						"release_date_precision": "day",
						"resume_point": {
							"fully_played": false,
							"resume_position_ms": 0
						},
						"type": "episode",
						"uri": "spotify:episode:0zLhl3WsOCQHbe1BPTiHgr",
						"restrictions": {
							"reason": "string"
						}
					}
				]
			},
			"audiobooks": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
						"authors": [
							{
								"name": "string"
							}
						],
						"available_markets": ["US"],
						"copyrights": [
							{
								"text": "string",
								"type": "C"
							}
						],
						"description": "string",
						"html_description": "string",
						"edition": "Unabridged",
						"explicit": false,
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
						"languages": ["string"],
						"media_type": "string",
						"name": "string",
						"narrators": [
							{
								"name": "string"
							}
						],
						"publisher": "string",
						"type": "audiobook",
						"uri": "string",
						"total_chapters": 0
					}
				]
			}
        }
        "#;

        crate::test::assert_deserialized!(SearchResults, json);
    }

    #[test]
    fn search_with_null() {
        let json = r#"
        {
			"playlists": {
				"href": "https://api.spotify.com/v1/search?offset=0&limit=3&query=weyes%20blood&type=playlist&locale=en-US,en;q%3D0.9",
				"limit": 3,
				"next": "https://api.spotify.com/v1/search?offset=3&limit=3&query=weyes%20blood&type=playlist&locale=en-US,en;q%3D0.9",
				"offset": 0,
				"previous": null,
				"total": 1000,
				"items": [
					{
						"collaborative": false,
						"description": "a selection of tracks&#x2F;artists heard in Natalie Mering&#x27;s radio shows or cited by her as influences on her music in interviews; also added some tracks that strongly remind me of her style and some of my favourite tracks of hers. playlist contains dream pop, soft rock, prog rock, soft jazz etc",
						"external_urls": {
							"spotify": "https://open.spotify.com/playlist/15BgYv4yT01R0WzCQIvovG"
						},
						"href": "https://api.spotify.com/v1/playlists/15BgYv4yT01R0WzCQIvovG",
						"id": "15BgYv4yT01R0WzCQIvovG",
						"images": [
							{
								"height": null,
								"url": "https://image-cdn-fa.spotifycdn.com/image/ab67706c0000da84ad34370274867428c199df89",
								"width": null
							}
						],
						"name": "weyes blood, queen of feminine harmonic sensuality",
						"owner": {
							"display_name": "shalante",
							"external_urls": {
								"spotify": "https://open.spotify.com/user/21hzwclnnfsqv5izwormhwqja"
							},
							"href": "https://api.spotify.com/v1/users/21hzwclnnfsqv5izwormhwqja",
							"id": "21hzwclnnfsqv5izwormhwqja",
							"type": "user",
							"uri": "spotify:user:21hzwclnnfsqv5izwormhwqja"
						},
						"primary_color": null,
						"public": true,
						"snapshot_id": "AAAANzRDyojBpbI4dp7oi8VXSTqYGiHU",
						"tracks": {
							"href": "https://api.spotify.com/v1/playlists/15BgYv4yT01R0WzCQIvovG/tracks",
							"total": 23
						},
						"type": "playlist",
						"uri": "spotify:playlist:15BgYv4yT01R0WzCQIvovG"
					},
					{
						"collaborative": false,
						"description": "continuously updating",
						"external_urls": {
							"spotify": "https://open.spotify.com/playlist/4M20pNFDm9AcD3EmQZS7v2"
						},
						"href": "https://api.spotify.com/v1/playlists/4M20pNFDm9AcD3EmQZS7v2",
						"id": "4M20pNFDm9AcD3EmQZS7v2",
						"images": [
							{
								"height": null,
								"url": "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da847aa80cdd93e3ef6d0519faca",
								"width": null
							}
						],
						"name": "ascending with weyes blood ",
						"owner": {
							"display_name": "emma",
							"external_urls": {
								"spotify": "https://open.spotify.com/user/31yrzgikh2zwd4xlfhfhlbckhkia"
							},
							"href": "https://api.spotify.com/v1/users/31yrzgikh2zwd4xlfhfhlbckhkia",
							"id": "31yrzgikh2zwd4xlfhfhlbckhkia",
							"type": "user",
							"uri": "spotify:user:31yrzgikh2zwd4xlfhfhlbckhkia"
						},
						"primary_color": null,
						"public": true,
						"snapshot_id": "AAAAucAZZRdsOpAfca28X80jqFiCy4FQ",
						"tracks": {
							"href": "https://api.spotify.com/v1/playlists/4M20pNFDm9AcD3EmQZS7v2/tracks",
							"total": 25
						},
						"type": "playlist",
						"uri": "spotify:playlist:4M20pNFDm9AcD3EmQZS7v2"
					},
					null
				]
			}
        }
        "#;

        crate::test::assert_deserialized!(SearchResults, json);
    }
}
