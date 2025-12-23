use crate::api::prelude::*;

/// Save one or more tracks to the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Clone)]
pub struct SaveTracksForCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for SaveTracksForCurrentUser
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl Endpoint for SaveTracksForCurrentUser {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/tracks".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("ids", &self.ids.join(","));
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };
    use http::Method;

    #[test]
    fn test_save_tracks_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/tracks")
            .add_query_params(&[("ids", "39joRyXYyjSpI6nKZHyWmH,5mPY98zmeNSp8cmrRtdUW3")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            SaveTracksForCurrentUser::from(["39joRyXYyjSpI6nKZHyWmH", "5mPY98zmeNSp8cmrRtdUW3"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
