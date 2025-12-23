use super::{Copyright, ExternalUrls, Image, ItemType, Market, Page, SimplifiedEpisode};
use serde::{Deserialize, Serialize};

/// Full show (podcast) information from the Spotify catalog.
///
/// Contains complete details about a show including its episodes
/// (when the `page_items` feature is enabled), description, publisher,
/// and available markets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Show {
    /// A list of the countries in which the show can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The copyright statements of the show.
    pub copyrights: Vec<Copyright>,

    /// A description of the show. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the show. This field may contain HTML tags.
    pub html_description: String,

    /// Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this show.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the show.
    pub href: String,

    /// The Spotify ID for the show.
    pub id: String,

    /// The cover art for the show in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if all of the show’s episodes are hosted outside of Spotify’s CDN.
    pub is_externally_hosted: bool,

    /// A list of the languages used in the episode, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The media type of the show.
    pub media_type: String,

    /// The name of the show.
    pub name: String,

    /// The publisher of the show.
    pub publisher: String,

    /// The object type.
    ///
    /// Allowed values: "show"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the show.
    pub uri: String,

    /// The total number of episodes in the show.
    pub total_episodes: usize,

    /// The episodes of the show.
    #[cfg(feature = "page_items")]
    pub episodes: Page<SimplifiedEpisode>,
}

/// Simplified show (podcast) information with basic details only.
///
/// A lighter version of [`Show`] that omits the episodes page.
/// Commonly returned when shows are nested within other objects like episodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedShow {
    /// A list of the countries in which the show can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The copyright statements of the show.
    pub copyrights: Vec<Copyright>,

    /// A description of the show. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the show. This field may contain HTML tags.
    pub html_description: String,

    /// Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this show.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the show.
    pub href: String,

    /// The Spotify ID for the show.
    pub id: String,

    /// The cover art for the show in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if all of the show’s episodes are hosted outside of Spotify’s CDN.
    pub is_externally_hosted: bool,

    /// A list of the languages used in the episode, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The media type of the show.
    pub media_type: String,

    /// The name of the show.
    pub name: String,

    /// The publisher of the show.
    pub publisher: String,

    /// The object type.
    ///
    /// Allowed values: "show"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the show.
    pub uri: String,

    /// The total number of episodes in the show.
    pub total_episodes: usize,
}

impl From<Show> for SimplifiedShow {
    fn from(show: Show) -> Self {
        Self {
            #[cfg(feature = "markets")]
            available_markets: show.available_markets,
            copyrights: show.copyrights,
            description: show.description,
            html_description: show.html_description,
            explicit: show.explicit,
            external_urls: show.external_urls,
            href: show.href,
            id: show.id,
            images: show.images,
            is_externally_hosted: show.is_externally_hosted,
            languages: show.languages,
            media_type: show.media_type,
            name: show.name,
            publisher: show.publisher,
            type_: show.type_,
            uri: show.uri,
            total_episodes: show.total_episodes,
        }
    }
}

/// A show saved to the current user's library.
///
/// Contains the timestamp when the show was saved and the show details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedShow {
    /// The date and time the show was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC)
    /// with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release),
    /// an additional field indicates the precision; see for example, `release_date` in an album object.
    pub added_at: String,

    /// Information about the show.
    pub show: SimplifiedShow,
}

/// Spotify catalog information for several shows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Shows {
    pub shows: Vec<SimplifiedShow>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show() {
        let json = r#"
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
			"total_episodes": 0,
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
			}
		}
        "#;

        crate::test::assert_deserialized!(Show, json);
    }

    #[test]
    fn simplified_show() {
        let json = r#"
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
        "#;

        crate::test::assert_deserialized!(SimplifiedShow, json);
    }
}
