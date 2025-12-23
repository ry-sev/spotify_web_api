use super::{Icon, Page};
use serde::{Deserialize, Serialize};

/// A category used to tag items in Spotify.
///
/// Categories appear in the Spotify player's "Browse" tab and can be used
/// to discover playlists and other content grouped by theme or genre.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Category {
    /// A link to the Web API endpoint returning full details of the category.
    pub href: String,

    /// The category icon, in various sizes.
    pub icons: Vec<Icon>,

    /// The [Spotify category ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the category.
    pub id: String,

    /// The name of the category.
    pub name: String,
}

/// A list of categories used to tag items in Spotify (on, for example, the Spotify player’s “Browse” tab).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Categories {
    pub categories: Page<Category>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category() {
        let json = r#"
        {
			"href": "string",
			"icons": [
				{
					"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
					"height": 300,
					"width": 300
				}
			],
			"id": "equal",
			"name": "EQUAL"
        }
        "#;

        crate::test::assert_deserialized!(Category, json);
    }
}
