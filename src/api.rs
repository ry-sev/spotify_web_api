mod client;
mod endpoint;
mod error;
mod ignore;
mod paged;
mod params;

pub(crate) mod query;

pub mod albums;
pub mod artists;
pub mod audiobooks;
pub mod categories;
pub mod chapters;
pub mod common;
pub mod episodes;
pub mod genres;
pub mod markets;
pub mod player;
pub mod search;
pub mod shows;
pub mod tracks;
pub mod users;

pub use client::{AsyncClient, Client, RestClient};
pub use endpoint::{Endpoint, UrlBase};
pub use error::{ApiError, BodyError};
pub use ignore::{ignore, Ignore};
pub use paged::*;
pub use params::{FormParams, JsonParams, ParamValue, QueryParams};
pub use query::{AsyncQuery, Query};

mod prelude {
    pub use super::Pageable;
    pub use crate::{
        api::{BodyError, JsonParams, QueryParams},
        model::Market,
    };
    pub use derive_builder::Builder;
    pub use endpoint_derive::Endpoint;
    pub use http::Method;
    pub use std::borrow::Cow;
}
