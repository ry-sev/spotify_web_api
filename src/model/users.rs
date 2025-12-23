use super::{ExternalUrls, Followers, Image, ItemType, Market};
use serde::{Deserialize, Serialize};

/// The user's Spotify subscription type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionType {
    #[serde(alias = "open")]
    Free,
    Premium,
}

impl std::fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Free => "Free",
            Self::Premium => "Premium",
        };
        write!(f, "{s}")
    }
}

/// The user's explicit content settings. This field is only available when the current user has granted access to the user-read-private scope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitContent {
    /// When true, indicates that explicit content should not be played.
    pub filter_enabled: bool,

    /// When true, indicates that the explicit content setting is locked and can't be changed by the user.
    pub filter_locked: bool,
}

/// The currently authenticated user's profile.
///
/// Contains additional private information compared to [`UserProfile`],
/// such as email and country (requires appropriate scopes).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentUserProfile {
    /// The country of the user, as set in the user's account profile.
    /// An [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// This field is only available when the current user has granted access to the [user-read-private scope](https://developer.spotify.com/documentation/web-api/concepts/scopes#list-of-scopes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Market>,

    /// The name displayed on the user's profile. None if not available.
    pub display_name: Option<String>,

    /// The user's email address, as entered by the user when creating their account.
    /// Important! This email address is unverified; there is no proof that it actually belongs to the user.
    /// This field is only available when the current user has granted access to the [user-read-email scope](https://developer.spotify.com/documentation/web-api/concepts/scopes#list-of-scopes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The user's explicit content settings.
    /// This field is only available when the current user has granted access to the user-read-private scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content: Option<ExplicitContent>,

    /// Known external URLs for this user.
    pub external_urls: ExternalUrls,

    /// Information about the followers of the user.
    pub followers: Followers,

    /// A link to the Web API endpoint for this user.
    pub href: String,

    /// The [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub id: String,

    /// The user's profile images.
    pub images: Vec<Image>,

    /// The user's Spotify subscription level: "premium", "free", etc.
    /// (The subscription level "open" can be considered the same as "free".)
    /// This field is only available when the current user has granted access to the [user-read-private scope](https://developer.spotify.com/documentation/web-api/concepts/scopes#list-of-scopes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<SubscriptionType>,

    /// The object type: "user".
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub uri: String,
}

/// A public user profile.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserProfile {
    /// The name displayed on the user's profile. None if not available.
    pub display_name: Option<String>,

    /// Known public external URLs for this user.
    pub external_urls: ExternalUrls,

    /// Information about the followers of this user.
    pub followers: Followers,

    /// A link to the Web API endpoint for this user.
    pub href: String,

    /// The [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub id: String,

    /// The user's profile images.
    pub images: Vec<Image>,

    /// The object type: "user"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub uri: String,
}

/// A simplified user reference (used in playlist ownership, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserReference {
    /// Known public external URLs for this user.
    pub external_urls: ExternalUrls,

    /// Information about the followers of this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Followers>,

    /// A link to the Web API endpoint for this user.
    pub href: String,

    /// The Spotify user ID for this user.
    pub id: String,

    /// The object type: "user"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the user.
    pub uri: String,

    /// The name displayed on the user's profile. None if not available.
    pub display_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_user_pofile() {
        let json = r#"
       	{
			"country": "US",
			"display_name": "string",
			"email": "string",
			"explicit_content": {
				"filter_enabled": false,
				"filter_locked": false
			},
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
			"product": "premium",
			"type": "user",
			"uri": "string"
        }
        "#;

        crate::test::assert_deserialized!(CurrentUserProfile, json);
    }

    #[test]
    fn user_profile() {
        let json = r#"
       	{
			"display_name": "string",
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
			"type": "user",
			"uri": "string"
        }
        "#;

        crate::test::assert_deserialized!(UserProfile, json);
    }
}
