use super::{
    Artist, Page, SimplifiedAlbum, SimplifiedAudiobook, SimplifiedEpisode, SimplifiedPlaylist,
    SimplifiedShow, Track,
};
use serde::{Deserialize, Serialize};

/// Spotify catalog information about albums, artists, playlists, tracks, shows, episodes or audiobooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlists: Option<Page<SimplifiedPlaylist>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Page<SimplifiedAlbum>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Page<Artist>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Page<Track>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shows: Option<Page<SimplifiedShow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Page<SimplifiedEpisode>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiobooks: Option<Page<SimplifiedAudiobook>>,
}

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
}
