use crate::{api::prelude::*, model::PlaylistItem};

/// Add one or more items to a user's playlist.
#[derive(Debug, Clone)]
pub struct AddItemsToPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,

    /// The position to insert the items, a zero-based index.
    /// For example, to insert the items in the first position: position=0; to insert the items in the third position: position=2.
    /// If omitted, the items will be appended to the playlist.
    /// Items are added in the order they are listed in the query string or request body.
    pub position: Option<u32>,

    /// A list of [Spotify URIs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) to set, can be track or episode URIs.
    pub uris: Vec<PlaylistItem>,
}

impl Endpoint for AddItemsToPlaylist {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("playlists/{}/tracks", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        let uris: String = self
            .uris
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join(",");

        params.push("uris", &uris);
        params.push_opt("position", self.position);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::TrackId,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_add_items_to_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/tracks")
            .add_query_params(&[("uris", "spotify:track:60zbztYPxtTQLLcPVjnEZG")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let track = TrackId::from_id("60zbztYPxtTQLLcPVjnEZG").unwrap();

        let endpoint = AddItemsToPlaylist {
            id: "3cEYpjA9oz9GiPac4AsH4n".to_owned(),
            position: None,
            uris: vec![track.into()],
        };

        api::ignore(endpoint).query(&client).unwrap();
    }
}
