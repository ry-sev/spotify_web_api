use super::{Copyright, ExternalUrls, Image, ItemType, Market, Page, SimplifiedChapter};
use serde::{Deserialize, Serialize};

/// An audiobook author.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    /// The name of the author.
    pub name: String,
}

/// An audiobook narrator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Narrator {
    /// The name of the Narrator.
    pub name: String,
}

/// Full audiobook information from the Spotify catalog.
///
/// Contains complete details about an audiobook including its chapters
/// (when the `page_items` feature is enabled), authors, narrators,
/// copyright information, and available markets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Audiobook {
    /// The author(s) for the audiobook.
    pub authors: Vec<Author>,

    /// A list of the countries in which the audiobook can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The copyright statements of the audiobook.
    pub copyrights: Vec<Copyright>,

    /// A description of the audiobook. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the audiobook. This field may contain HTML tags.
    pub html_description: String,

    /// The edition of the audiobook.
    pub edition: String,

    /// Whether or not the audiobook has explicit content
    /// (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this audiobook.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the audiobook.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
    pub id: String,

    /// The cover art for the audiobook in various sizes, widest first.
    pub images: Vec<Image>,

    /// A list of the languages used in the audiobook, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The media type of the audiobook.
    pub media_type: String,

    /// The name of the audiobook.
    pub name: String,

    /// The narrator(s) for the audiobook.
    pub narrators: Vec<Narrator>,

    /// The publisher of the audiobook.
    pub publisher: String,

    /// The object type.
    ///
    /// Allowed values: "audiobook"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
    pub uri: String,

    /// The number of chapters in this audiobook.
    pub total_chapters: usize,

    /// A list of simplified chapters in this audiobook.
    #[cfg(feature = "page_items")]
    pub chapters: Page<SimplifiedChapter>,
}

/// Simplified audiobook information with basic details only.
///
/// A lighter version of [`Audiobook`] that omits the chapters page.
/// Commonly returned when audiobooks are nested within other objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAudiobook {
    /// The author(s) for the audiobook.
    pub authors: Vec<Author>,

    /// A list of the countries in which the audiobook can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The copyright statements of the audiobook.
    pub copyrights: Vec<Copyright>,

    /// A description of the audiobook. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the audiobook. This field may contain HTML tags.
    pub html_description: String,

    /// The edition of the audiobook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,

    /// Whether or not the audiobook has explicit content
    /// (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this audiobook.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the audiobook.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
    pub id: String,

    /// The cover art for the audiobook in various sizes, widest first.
    pub images: Vec<Image>,

    /// A list of the languages used in the audiobook, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The media type of the audiobook.
    pub media_type: String,

    /// The name of the audiobook.
    pub name: String,

    /// The narrator(s) for the audiobook.
    pub narrators: Vec<Narrator>,

    /// The publisher of the audiobook.
    pub publisher: String,

    /// The object type.
    ///
    /// Allowed values: "audiobook"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
    pub uri: String,

    /// The number of chapters in this audiobook.
    pub total_chapters: usize,
}

impl From<Audiobook> for SimplifiedAudiobook {
    fn from(audiobook: Audiobook) -> Self {
        Self {
            authors: audiobook.authors,
            #[cfg(feature = "markets")]
            available_markets: audiobook.available_markets,
            copyrights: audiobook.copyrights,
            description: audiobook.description,
            html_description: audiobook.html_description,
            edition: Some(audiobook.edition),
            explicit: audiobook.explicit,
            external_urls: audiobook.external_urls,
            href: audiobook.href,
            id: audiobook.id,
            images: audiobook.images,
            languages: audiobook.languages,
            media_type: audiobook.media_type,
            name: audiobook.name,
            narrators: audiobook.narrators,
            publisher: audiobook.publisher,
            type_: audiobook.type_,
            uri: audiobook.uri,
            total_chapters: audiobook.total_chapters,
        }
    }
}

/// Spotify catalog information for several audiobooks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Audiobooks {
    pub audiobooks: Vec<Option<Audiobook>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audiobook() {
        let json = r#"
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
			"total_chapters": 0,
			"chapters": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"offset": 0,
				"previous": "https://api.spotify.com/v1/me/shows?offset=1&limit=1",
				"total": 4,
				"items": [
					{
						"audio_preview_url": "https://p.scdn.co/mp3-preview/2f37da1d4221f40b9d1a98cd191f4d6f1646ad17",
						"available_markets": ["US"],
						"chapter_number": 1,
						"description": "We kept on ascending, with occasional periods of quick descent, but in the main always ascending. Suddenly, I became conscious of the fact that the driver was in the act of pulling up the horses in the courtyard of a vast ruined castle, from whose tall black windows came no ray of light, and whose broken battlements showed a jagged line against the moonlit sky.",
						"html_description": "<p>We kept on ascending, with occasional periods of quick descent, but in the main always ascending. Suddenly, I became conscious of the fact that the driver was in the act of pulling up the horses in the courtyard of a vast ruined castle, from whose tall black windows came no ray of light, and whose broken battlements showed a jagged line against the moonlit sky.</p>",
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
						"is_playable": false,
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

        crate::test::assert_deserialized!(Audiobook, json);
    }

    #[test]
    fn simplified_audiobook() {
        let json = r#"
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
        "#;

        crate::test::assert_deserialized!(SimplifiedAudiobook, json);
    }
}
