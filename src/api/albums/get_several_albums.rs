use crate::api::prelude::*;

/// Get Spotify catalog information for multiple albums identified by their Spotify IDs.
#[derive(Debug, Clone)]
pub struct GetSeveralAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    pub ids: Vec<String>,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl<T, I> From<I> for GetSeveralAlbums
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            market: None,
        }
    }
}

impl Endpoint for GetSeveralAlbums {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "albums".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("ids", &self.ids.join(","));
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
    fn test_get_several_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("albums")
            .add_query_params(&[(
                "ids",
                "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc",
            )])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralAlbums::from([
            "382ObEPsp2rxGrnsizN5TX",
            "1A2GTWGtFfWp7KSQTwWOyo",
            "2noRn2Aes5aoNVsU6iWThc",
        ]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
