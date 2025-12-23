use super::{
    ExternalUrls, Image, ItemType, ReleaseDatePrecision, Restrictions, ResumePoint, SimplifiedShow,
};
use serde::{Deserialize, Serialize};

/// Full episode (podcast episode) information from the Spotify catalog.
///
/// Contains complete details about an episode including its parent show,
/// release date, duration, and playback resume point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Episode {
    /// A description of the episode. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the episode. This field may contain HTML tags.
    pub html_description: String,

    /// The episode length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the episode has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this episode.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the episode.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episode.
    pub id: String,

    /// The cover art for the episode in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if the episode is hosted outside of Spotify's CDN.
    pub is_externally_hosted: bool,

    /// True if the episode is playable in the given market. Otherwise false.
    pub is_playable: bool,

    /// A list of the languages used in the episode, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The name of the episode.
    pub name: String,

    /// The date the episode was first released, for example "1981-12-15".
    /// Depending on the precision, it might be shown as "1981" or "1981-12".
    pub release_date: String,

    /// The precision with which `release_date` value is known.
    pub release_date_precision: ReleaseDatePrecision,

    /// The user's most recent position in the episode.
    /// Set if the supplied access token is a user token and has the scope 'user-read-playback-position'.
    pub resume_point: Option<ResumePoint>,

    /// The object type.
    ///
    /// Allowed values: "episode"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episode.
    pub uri: String,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The show on which the episode belongs.
    pub show: SimplifiedShow,
}

/// Simplified episode information with basic details only.
///
/// A lighter version of [`Episode`] that omits the parent show.
/// Commonly returned when episodes are nested within other objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedEpisode {
    /// A description of the episode. HTML tags are stripped away from this field,
    /// use `html_description` field in case HTML tags are needed.
    pub description: String,

    /// A description of the episode. This field may contain HTML tags.
    pub html_description: String,

    /// The episode length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the episode has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// External URLs for this episode.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the episode.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episode.
    pub id: String,

    /// The cover art for the episode in various sizes, widest first.
    pub images: Vec<Image>,

    /// True if the episode is hosted outside of Spotify's CDN.
    pub is_externally_hosted: bool,

    /// True if the episode is playable in the given market. Otherwise false.
    pub is_playable: bool,

    /// A list of the languages used in the episode, identified by their [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,

    /// The name of the episode.
    pub name: String,

    /// The date the episode was first released, for example "1981-12-15".
    /// Depending on the precision, it might be shown as "1981" or "1981-12".
    pub release_date: String,

    /// The precision with which `release_date` value is known.
    pub release_date_precision: ReleaseDatePrecision,

    /// The user's most recent position in the episode.
    /// Set if the supplied access token is a user token and has the scope 'user-read-playback-position'.
    pub resume_point: Option<ResumePoint>,

    /// The object type.
    ///
    /// Allowed values: "episode"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episode.
    pub uri: String,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
}

impl From<Episode> for SimplifiedEpisode {
    fn from(episode: Episode) -> Self {
        Self {
            description: episode.description,
            html_description: episode.html_description,
            duration_ms: episode.duration_ms,
            explicit: episode.explicit,
            external_urls: episode.external_urls,
            href: episode.href,
            id: episode.id,
            images: episode.images,
            is_externally_hosted: episode.is_externally_hosted,
            is_playable: episode.is_playable,
            languages: episode.languages,
            name: episode.name,
            release_date: episode.release_date,
            release_date_precision: episode.release_date_precision,
            resume_point: episode.resume_point,
            type_: episode.type_,
            uri: episode.uri,
            restrictions: episode.restrictions,
        }
    }
}

/// Spotify catalog information for several episodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Episodes {
    pub episodes: Vec<Option<Episode>>,
}

/// An episode saved to the current user's library.
///
/// Contains the timestamp when the episode was saved and the episode details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedEpisode {
    /// The date and time the episode was saved.
    /// Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ.
    pub added_at: String,

    /// Information about the episode.
    pub episode: Episode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode() {
        let json = r#"
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
			},
			"show": {
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
        }
        "#;

        crate::test::assert_deserialized!(Episode, json);
    }

    #[test]
    fn simplified_episode() {
        let json = r#"
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
			},
			"show": {
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
		}
        "#;

        crate::test::assert_deserialized!(SimplifiedEpisode, json);
    }
}
