use crate::api::prelude::*;

/// Save one or more audiobooks to the current Spotify user's library.
#[derive(Debug, Clone)]
pub struct SaveAudiobooksForCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobooks.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for SaveAudiobooksForCurrentUser
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

impl Endpoint for SaveAudiobooksForCurrentUser {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/audiobooks".into()
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
    fn test_save_audiobooks_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/audiobooks")
            .add_query_params(&[(
                "ids",
                "18yVqkdbdRvS24c0Ilj2ci,1HGw3J3NxZO1TP1BTtVhpZ,7iHfbu1YPACw6oZPAFJtqe",
            )])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SaveAudiobooksForCurrentUser::from([
            "18yVqkdbdRvS24c0Ilj2ci",
            "1HGw3J3NxZO1TP1BTtVhpZ",
            "7iHfbu1YPACw6oZPAFJtqe",
        ]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
