use crate::api::prelude::*;

/// Get Spotify catalog information for a single show identified by its unique Spotify ID.
#[derive(Debug, Clone)]
pub struct GetShow {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the show.
    pub id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl<T: Into<String>> From<T> for GetShow {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
        }
    }
}

impl Endpoint for GetShow {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("shows/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("market", self.market.as_ref());
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

    #[test]
    fn test_get_show_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("shows/38bS44xjbVVZ3No3ByF1dJ")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetShow::from("38bS44xjbVVZ3No3ByF1dJ");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
