use crate::api::prelude::*;

/// Get a list of the playlists owned or followed by the current Spotify user.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/playlists")]
pub struct GetCurrentUserPlaylists;

impl Pageable for GetCurrentUserPlaylists {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_current_user_playlists_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/playlists")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetCurrentUserPlaylists).query(&client).unwrap();
    }
}
