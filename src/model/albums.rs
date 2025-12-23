use super::{
    Copyright, ExternalIds, ExternalUrls, Image, ItemType, Market, Page, ReleaseDatePrecision,
    Restrictions, SimplifiedArtist, SimplifiedTrack,
};
use serde::{Deserialize, Serialize};

/// The type of an album.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    #[serde(alias = "ALBUM")]
    Album,
    #[serde(alias = "SINGLE")]
    Single,
    #[serde(alias = "COMPILATION")]
    Compilation,
    #[serde(alias = "APPEARS_ON")]
    AppearsOn,
}

impl std::fmt::Display for AlbumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Album => "Album",
            Self::Single => "Single",
            Self::Compilation => "Compilation",
            Self::AppearsOn => "Appears on",
        };
        write!(f, "{s}")
    }
}

impl AlbumType {
    pub fn all() -> &'static [Self] {
        &[
            Self::Album,
            Self::Single,
            Self::Compilation,
            Self::AppearsOn,
        ]
    }

    pub fn snake_case(&self) -> &'static str {
        match self {
            Self::Album => "album",
            Self::Single => "single",
            Self::Compilation => "compilation",
            Self::AppearsOn => "appears_on",
        }
    }
}

/// A full album object from the Spotify catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Album {
    /// The type of the album.
    pub album_type: AlbumType,

    /// The number of tracks in the album.
    pub total_tracks: usize,

    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    ///
    /// # Note
    /// An album is considered available in a market when at least 1 of its tracks is available in that market.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// Known external URLs for this album.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the album.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the album.
    pub id: String,

    /// The cover art for the album in various sizes, widest first.
    pub images: Vec<Image>,

    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub name: String,

    /// The date the album was first released.
    ///
    /// Example: "1981-12"
    pub release_date: String,

    /// The precision with which `release_date` value is known.
    pub release_date_precision: ReleaseDatePrecision,

    /// Included in the response when a content restriction is applied.
    /// Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content. Additional reasons may be added in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The object type.
    ///
    /// Allowed values: "album"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the album.
    pub uri: String,

    /// The artists of the album. Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SimplifiedArtist>,

    /// The tracks of the album.
    #[cfg(feature = "page_items")]
    pub tracks: Page<SimplifiedTrack>,

    /// The copyright statements of the album.
    pub copyrights: Vec<Copyright>,

    /// Known external IDs for the album.
    pub external_ids: ExternalIds,

    /// The label associated with the album.
    pub label: String,

    /// The popularity of the album. The value will be between 0 and 100, with 100 being the most popular.
    pub popularity: u8,

    /// This field describes the relationship between the artist and the album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<AlbumType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAlbum {
    /// The type of the album.
    pub album_type: AlbumType,

    /// The number of tracks in the album.
    pub total_tracks: usize,

    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    ///
    /// # Note
    /// An album is considered available in a market when at least 1 of its tracks is available in that market.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// Known external URLs for this album.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the album.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the album.
    pub id: String,

    /// The cover art for the album in various sizes, widest first.
    pub images: Vec<Image>,

    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub name: String,

    /// The date the album was first released.
    ///
    /// Example: "1981-12"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,

    /// The precision with which `release_date` value is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<ReleaseDatePrecision>,

    /// Included in the response when a content restriction is applied.
    /// Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content. Additional reasons may be added in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The object type.
    ///
    /// Allowed values: "album"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the album.
    pub uri: String,

    /// The artists of the album. Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SimplifiedArtist>,
}

impl From<Album> for SimplifiedAlbum {
    fn from(album: Album) -> Self {
        Self {
            album_type: album.album_type,
            total_tracks: album.total_tracks,
            #[cfg(feature = "markets")]
            available_markets: album.available_markets,
            external_urls: album.external_urls,
            href: album.href,
            id: album.id,
            images: album.images,
            name: album.name,
            release_date: Some(album.release_date),
            release_date_precision: Some(album.release_date_precision),
            restrictions: album.restrictions,
            type_: album.type_,
            uri: album.uri,
            artists: album.artists,
        }
    }
}

/// Spotify catalog information for several albums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Albums {
    pub albums: Vec<Option<Album>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedAlbum {
    /// The date and time the track was saved.
    /// Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ.
    /// If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision;
    /// see for example, `release_date` in an album object.
    pub added_at: String,

    /// Information about the album.
    pub album: Album,
}

/// A list of new album releases featured in Spotify (shown, for example, on a Spotify player’s “Browse” tab).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewReleases {
    pub albums: Page<SimplifiedAlbum>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn album() {
        let json = r#"
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
			],
			"tracks": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
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
						"available_markets": ["CA", "BR", "IT"],
						"disc_number": 0,
						"duration_ms": 0,
						"explicit": false,
						"external_urls": {
							"spotify": "string"
						},
						"href": "string",
						"id": "string",
						"is_playable": false,
						"linked_from": {
							"external_urls": {
								"spotify": "string"
							},
							"href": "string",
							"id": "string",
							"type": "track",
							"uri": "string"
						},
						"restrictions": {
							"reason": "string"
						},
						"name": "string",
						"preview_url": "string",
						"track_number": 0,
						"type": "track",
						"uri": "string",
						"is_local": false
					}
				]
			},
			"copyrights": [
				{
					"text": "string",
					"type": "P"
				}
			],
			"external_ids": {
				"isrc": "string",
				"ean": "string",
				"upc": "string"
			},
			"genres": ["Egg punk", "Noise rock"],
			"label": "string",
			"popularity": 0
        }
        "#;

        crate::test::assert_deserialized!(Album, json);
    }

    #[test]
    fn simplified_album() {
        let json = r#"
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
        "#;

        crate::test::assert_deserialized!(SimplifiedAlbum, json);
    }
}
