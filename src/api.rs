//! Spotify Web API endpoints.
//!
//! This module contains endpoint definitions for all Spotify Web API operations,
//! organized by resource type (albums, artists, playlists, etc.).
//!
//! # Usage
//!
//! Each endpoint is represented as a struct that implements the [`Endpoint`] trait.
//! Endpoints can be executed using the [`Query`] or [`AsyncQuery`] traits.
//!
//! # Modules
//!
//! - [`albums`] - Album-related endpoints
//! - [`artists`] - Artist-related endpoints
//! - [`audiobooks`] - Audiobook-related endpoints
//! - [`categories`] - Browse category endpoints
//! - [`chapters`] - Audiobook chapter endpoints
//! - [`episodes`] - Podcast episode endpoints
//! - [`genres`] - Genre-related endpoints
//! - [`markets`] - Market availability endpoints
//! - [`player`] - Playback control endpoints
//! - [`playlists`] - Playlist-related endpoints
//! - [`search`] - Search endpoints
//! - [`shows`] - Podcast show endpoints
//! - [`tracks`] - Track-related endpoints
//! - [`users`] - User profile and follow endpoints

mod client;
mod endpoint;
mod error;
mod ignore;
mod paged;
mod params;
mod raw;

pub mod common;
pub(crate) mod query;

pub mod albums;
pub mod artists;
pub mod audiobooks;
pub mod categories;
pub mod chapters;
pub mod episodes;
pub mod genres;
pub mod markets;
pub mod player;
pub mod playlists;
pub mod search;
pub mod shows;
pub mod tracks;
pub mod users;

pub use client::*;
pub use endpoint::*;
pub use error::*;
pub use ignore::*;
pub use paged::*;
pub use params::*;
pub use query::{AsyncQuery, Query};
pub use raw::*;

mod prelude {
    pub use super::Pageable;
    pub use crate::{
        api::{BodyError, Endpoint, JsonParams, QueryParams},
        model::Market,
    };
    pub use http::Method;
    pub use std::borrow::Cow;
}
