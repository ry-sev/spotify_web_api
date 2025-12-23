use thiserror::Error;

/// Errors that can occur when parsing or validating Spotify IDs.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("The ID is not in correct format.")]
    InvalidFormat,

    #[error("The ID is not the correct length. Got {got}, expected {expected}.")]
    InvalidLength { got: usize, expected: usize },
}

/// The type of a Spotify resource identified by an ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdType {
    User,
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::User => "user",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Show => "show",
            Self::Episode => "episode",
        };
        write!(f, "{s}")
    }
}

/// A playback context type with its associated ID.
///
/// Represents items that can be used as a playback context (the source from
/// which tracks are played), such as an album, artist, playlist, or show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextType {
    Album(AlbumId),
    Artist(ArtistId),
    Playlist(PlaylistId),
    Show(ShowId),
}

impl ContextType {
    pub fn uri(&self) -> String {
        match self {
            Self::Album(id) => id.uri(),
            Self::Artist(id) => id.uri(),
            Self::Playlist(id) => id.uri(),
            Self::Show(id) => id.uri(),
        }
    }
}

impl From<AlbumId> for ContextType {
    fn from(id: AlbumId) -> Self {
        Self::Album(id)
    }
}

impl From<ArtistId> for ContextType {
    fn from(id: ArtistId) -> Self {
        Self::Artist(id)
    }
}

impl From<PlaylistId> for ContextType {
    fn from(id: PlaylistId) -> Self {
        Self::Playlist(id)
    }
}

impl From<ShowId> for ContextType {
    fn from(id: ShowId) -> Self {
        Self::Show(id)
    }
}

macro_rules! impl_ids {
    ($(#[doc = $doc:literal] ($struct_name:ident, $id_type:ident, $type_name:expr)),* $(,)?) => {
        $(
            #[doc = $doc]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $struct_name(String);

            impl $struct_name {
                /// Creates an ID from a base-62 Spotify identifier string.
                ///
                /// # Errors
                /// Returns [`IdError::InvalidLength`] if the ID is not 22 characters.
                /// Returns [`IdError::InvalidFormat`] if the ID contains non-alphanumeric characters.
                pub fn from_id<S>(id: S) -> Result<Self, IdError> where S: Into<String> {
                    let id = id.into();

                    if $type_name == "user" {
                        return Ok($struct_name(id.into()));
                    }

                    let id_len = id.len();

                    match id_len {
                        22 if is_base62(&id) => Ok($struct_name(id.into())),
                        22 => Err(IdError::InvalidFormat),
                        _ => Err(IdError::InvalidLength {
                        	got: id_len,
                         	expected: 22,
                        }),
                    }
                }

                /// Creates an ID from a Spotify URI string (e.g., `spotify:track:6rqhFgbbKwnb9MLmUQDhG6`).
                ///
                /// # Errors
                /// Returns [`IdError::InvalidFormat`] if the URI doesn't have the expected prefix.
                /// Returns [`IdError::InvalidLength`] if the extracted ID is not 22 characters.
                pub fn from_uri<S>(uri: S) -> Result<Self, IdError> where S: Into<String> {
					let uri = uri.into();
					let prefix = format!("spotify:{}:", $type_name);

					let id = uri.strip_prefix(&prefix).ok_or(IdError::InvalidFormat)?;

					if $type_name == "user" {
                        return Ok($struct_name(id.into()));
                    }

					let id_len = id.len();

					match id_len {
						22 if is_base62(&id) => Ok($struct_name(id.into())),
						22 => Err(IdError::InvalidFormat),
						_ => Err(IdError::InvalidLength {
							got: id_len,
							expected: 22,
						}),
					}
				}

                /// The base-62 identifier found at the end of the Spotify URI (see above) for an artist, track, album, playlist, etc.
                /// Unlike a Spotify URI, a Spotify ID does not clearly identify the type of resource; that information is provided elsewhere in the call.
                pub fn id(&self) -> &str {
					&self.0
				}

				/// The type of the resource.
				pub fn _type(&self) -> IdType {
					IdType::$id_type
				}

				/// The resource identifier of, for example, an artist, album or track.
				pub fn uri(&self) -> String {
        			format!("spotify:{}:{}", self._type(), self.id())
    			}
            }
        )*
    }
}

impl_ids![
    #[doc = "A validated Spotify playlist ID."]
    (PlaylistId, Playlist, "playlist"),
    #[doc = "A validated Spotify track ID."]
    (TrackId, Track, "track"),
    #[doc = "A validated Spotify album ID."]
    (AlbumId, Album, "album"),
    #[doc = "A validated Spotify artist ID."]
    (ArtistId, Artist, "artist"),
    #[doc = "A validated Spotify show (podcast) ID."]
    (ShowId, Show, "show"),
    #[doc = "A validated Spotify episode ID."]
    (EpisodeId, Episode, "episode"),
    #[doc = "A Spotify user ID."]
    (UserId, User, "user"),
];

#[inline(always)]
fn is_base62(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let id = "6rqhFgbbKwnb9MLmUQDhG6";
        let track_id = TrackId::from_id(id).unwrap();
        assert_eq!(track_id.id(), id);

        let bad_id = "6rqhFgbbKwnb9MLmUQDhG";
        let track_id = TrackId::from_id(bad_id);
        assert_eq!(
            track_id,
            Err(IdError::InvalidLength {
                got: 21,
                expected: 22
            })
        );
    }

    #[test]
    fn test_id_from_uri() {
        let uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
        let track_id = TrackId::from_uri(uri).unwrap();
        assert_eq!(track_id.id(), "6rqhFgbbKwnb9MLmUQDhG6");

        let bad_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG";
        let track_id = TrackId::from_uri(bad_uri);
        assert_eq!(
            track_id,
            Err(IdError::InvalidLength {
                got: 21,
                expected: 22
            })
        );
    }
}
