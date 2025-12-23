use crate::api::prelude::*;

/// Save one or more shows to the current Spotify user's library.
#[derive(Debug, Clone)]
pub struct SaveShowsForCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the shows.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for SaveShowsForCurrentUser
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

impl Endpoint for SaveShowsForCurrentUser {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/shows".into()
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
    fn test_save_shows_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/shows")
            .add_query_params(&[("ids", "5CfCWKI5pZ28U0uOzXkDHe,5as3aKmN2k11yfDDDSrvaZ")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            SaveShowsForCurrentUser::from(["5CfCWKI5pZ28U0uOzXkDHe", "5as3aKmN2k11yfDDDSrvaZ"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
