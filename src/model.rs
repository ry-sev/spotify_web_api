//! Spotify Web API data model.
//!
//! This module contains all data types returned by the Spotify Web API,
//! organized by resource type.
//!
//! # Feature Flags
//!
//! Some fields are conditionally compiled based on feature flags:
//!
//! - `markets` (default): Includes `available_markets` fields on tracks, albums, etc.
//! - `page_items` (default): Includes `items` fields on paginated responses.
//!
//! # Common Types
//!
//! - [`Page`] - Paginated response wrapper
//! - [`Image`] - Image metadata
//! - [`ExternalUrls`] - External URL links (Spotify URLs)
//! - [`Followers`] - Follower information

pub mod albums;
pub mod artists;
pub mod audiobooks;
pub mod categories;
pub mod chapters;
pub mod episodes;
pub mod genres;
pub mod id;
pub mod markets;
pub mod misc;
pub mod player;
pub mod playlists;
pub mod search;
pub mod shows;
pub mod token;
pub mod tracks;
pub mod users;

pub use albums::*;
pub use artists::*;
pub use audiobooks::*;
pub use categories::*;
pub use chapters::*;
pub use episodes::*;
pub use genres::*;
pub use id::*;
pub use markets::*;
pub use misc::*;
pub use player::*;
pub use playlists::*;
pub use search::*;
pub use shows::*;
pub use token::*;
pub use tracks::*;
pub use users::*;
