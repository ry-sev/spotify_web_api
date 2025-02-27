use crate::api::prelude::*;

/// Check if one or more albums is already saved in the current Spotify user's 'Your Music' library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/albums/contains")]
pub struct CheckUserSavedAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    pub ids: Vec<String>,
}

impl CheckUserSavedAlbumsBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedAlbums {
    pub fn builder() -> CheckUserSavedAlbumsBuilder {
        CheckUserSavedAlbumsBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::Query as _,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_check_user_saved_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/albums/contains")
            .add_query_params(&[(
                "ids",
                "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc",
            )])
            .build()
            .unwrap();

        let expected_response = [false, false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedAlbums::builder()
            .id("382ObEPsp2rxGrnsizN5TX")
            .id("1A2GTWGtFfWp7KSQTwWOyo")
            .id("2noRn2Aes5aoNVsU6iWThc")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
