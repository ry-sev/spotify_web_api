use crate::api::prelude::*;

/// Save one or more episodes to the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Clone)]
pub struct SaveEpisodesForCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episodes.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for SaveEpisodesForCurrentUser
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

impl Endpoint for SaveEpisodesForCurrentUser {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/episodes".into()
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
    fn test_save_episodes_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/episodes")
            .add_query_params(&[("ids", "77o6BIVlYM3msb4MMIL1jH,0Q86acNRm6V9GYx55SXKwf")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            SaveEpisodesForCurrentUser::from(["77o6BIVlYM3msb4MMIL1jH", "0Q86acNRm6V9GYx55SXKwf"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
