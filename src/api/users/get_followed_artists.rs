use crate::{api::prelude::*, model::FollowedArtistsType};

/// Get the current user's followed artists.
#[derive(Debug, Clone)]
pub struct GetFollowedArtists {
    /// The ID type: currently only artist is supported.
    pub type_: FollowedArtistsType,

    /// The last artist ID retrieved from the previous request.
    pub after: Option<String>,
}

impl GetFollowedArtists {
    pub fn with_after(after: Option<impl Into<String>>) -> Self {
        Self {
            type_: FollowedArtistsType::Artist,
            after: after.map(Into::into),
        }
    }
}

impl Default for GetFollowedArtists {
    fn default() -> Self {
        Self {
            type_: FollowedArtistsType::Artist,
            after: None,
        }
    }
}

impl From<FollowedArtistsType> for GetFollowedArtists {
    fn from(type_: FollowedArtistsType) -> Self {
        Self { type_, after: None }
    }
}

impl Endpoint for GetFollowedArtists {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/following".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("type", &self.type_);
        params.push_opt("after", self.after.as_ref());
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
    fn test_get_followed_artists_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetFollowedArtists::default())
            .query(&client)
            .unwrap();
    }

    #[test]
    fn test_get_followed_artists_endpoint_with_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .add_query_params(&[("after", "2CIMQHirSU0MQqyYHq0eOx")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetFollowedArtists::with_after(Some(
            "2CIMQHirSU0MQqyYHq0eOx",
        )))
        .query(&client)
        .unwrap();
    }
}
