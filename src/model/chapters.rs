use super::{
    ExternalUrls, Image, ItemType, Market, ReleaseDatePrecision, Restrictions, ResumePoint,
    SimplifiedAudiobook,
};
use serde::{Deserialize, Serialize};

/// Full audiobook chapter information from the Spotify catalog.
///
/// Contains complete details about a chapter including its parent audiobook,
/// chapter number, release date, duration, and playback resume point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chapter {
    /// A list of the countries in which the audiobook can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The number of the chapter.
    pub chapter_number: u32,

    /// A description of the chapter. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the chapter. This field may contain HTML tags.
    pub html_description: String,

    /// The chapter length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the chapter has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this chapter.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the chapter.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
    pub id: String,

    /// The cover art for the chapter in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if the chapter is playable in the given market. Otherwise false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,

    /// A list of the languages used in the chapter, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The name of the chapter.
    pub name: String,

    /// The date the chapter was first released, for example "1981-12-15".
    pub release_date: String,

    /// The precision with which `release_date` value is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<ReleaseDatePrecision>,

    /// The user's most recent position in the chapter.
    /// Set if the supplied access token is a user token and has the scope 'user-read-playback-position'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_point: Option<ResumePoint>,

    /// The object type.
    ///
    /// Allowed values: "episode"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
    pub uri: String,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The audiobook for which the chapter belongs.
    pub audiobook: SimplifiedAudiobook,
}

/// Simplified audiobook chapter information with basic details only.
///
/// A lighter version of [`Chapter`] that omits the parent audiobook.
/// Commonly returned when chapters are nested within audiobook objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedChapter {
    /// A list of the countries in which the audiobook can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    #[cfg(feature = "markets")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<Market>,

    /// The number of the chapter.
    pub chapter_number: u32,

    /// A description of the chapter. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the chapter. This field may contain HTML tags.
    pub html_description: String,

    /// The chapter length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the chapter has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this chapter.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the chapter.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
    pub id: String,

    /// The cover art for the chapter in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if the chapter is playable in the given market. Otherwise false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,

    /// A list of the languages used in the chapter, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The name of the chapter.
    pub name: String,

    /// The date the chapter was first released, for example "1981-12-15".
    pub release_date: String,

    /// The precision with which `release_date` value is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<ReleaseDatePrecision>,

    /// The user's most recent position in the chapter.
    /// Set if the supplied access token is a user token and has the scope 'user-read-playback-position'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_point: Option<ResumePoint>,

    /// The object type.
    ///
    /// Allowed values: "episode"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the chapter.
    pub uri: String,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
}

impl From<Chapter> for SimplifiedChapter {
    fn from(chapter: Chapter) -> Self {
        Self {
            #[cfg(feature = "markets")]
            available_markets: chapter.available_markets,
            chapter_number: chapter.chapter_number,
            description: chapter.description,
            html_description: chapter.html_description,
            duration_ms: chapter.duration_ms,
            explicit: chapter.explicit,
            external_urls: chapter.external_urls,
            href: chapter.href,
            id: chapter.id,
            images: chapter.images,
            is_playable: chapter.is_playable,
            languages: chapter.languages,
            name: chapter.name,
            release_date: chapter.release_date,
            release_date_precision: chapter.release_date_precision,
            resume_point: chapter.resume_point,
            type_: chapter.type_,
            uri: chapter.uri,
            restrictions: chapter.restrictions,
        }
    }
}

/// Spotify catalog information for several audiobook chapters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chapters {
    pub chapters: Vec<Option<Chapter>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chapter() {
        let json = r#"
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
			},
			"audiobook": {
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
        }
        "#;

        crate::test::assert_deserialized!(Chapter, json);
    }

    #[test]
    fn simplified_chapter() {
        let json = r#"
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
        "#;

        crate::test::assert_deserialized!(SimplifiedChapter, json);
    }
}
