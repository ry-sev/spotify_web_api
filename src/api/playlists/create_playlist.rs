use crate::api::prelude::*;
use serde_json::json;

/// Create a playlist for a Spotify user.
/// (The playlist will be empty until you [add tracks](https://developer.spotify.com/documentation/web-api/reference/add-tracks-to-playlist).)
/// Each user is generally limited to a maximum of 11000 playlists.
#[derive(Debug, Clone)]
pub struct CreatePlaylist {
    /// The user's [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    pub id: String,

    /// The new name for the playlist, for example "My New Playlist Title".
    pub name: String,

    /// The playlist's public/private status (if it should be added to the user's profile or not):
    /// true the playlist will be public,
    /// false the playlist will be private, null the playlist status is not relevant.
    /// For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/web-api/concepts/playlists).
    pub public: Option<bool>,

    /// If true, the playlist will become collaborative and other users will be able to modify the playlist in their Spotify client.
    /// # Note:
    /// You can only set collaborative to true on non-public playlists.
    pub collaborative: Option<bool>,

    /// Value for playlist description as displayed in Spotify Clients and in the Web API.
    pub description: Option<String>,
}

impl Endpoint for CreatePlaylist {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/playlists", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut body = json!({
            "name": self.name,
        });

        if let Some(public) = self.public {
            body["public"] = json!(public);
        }

        if let Some(collaborative) = self.collaborative {
            body["collaborative"] = json!(collaborative);
        }

        if let Some(description) = &self.description {
            body["description"] = json!(description);
        }

        JsonParams::into_body(&body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_create_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .content_type("application/json")
            .endpoint("users/smedjan/playlists")
            .body_str(r#"{"description":"New playlist description","name":"New Playlist","public":false}"#)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreatePlaylist {
            id: "smedjan".to_owned(),
            name: "New Playlist".to_owned(),
            description: Some("New playlist description".to_owned()),
            public: Some(false),
            collaborative: None,
        };

        api::ignore(endpoint).query(&client).unwrap();
    }
}
