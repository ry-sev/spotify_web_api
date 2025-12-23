//! Types used in the authorization process.
//!
//! This module provides types for OAuth 2.0 authorization flows supported by the Spotify API:
//!
//! - **Authorization Code with PKCE**: For applications that need to access user data.
//!   Use [`crate::SpotifyPKCE`] or [`crate::AsyncSpotifyPKCE`].
//! - **Client Credentials**: For server-to-server authentication without user context.
//!   Use [`crate::SpotifyClientCredentials`] or [`crate::AsyncSpotifyClientCredentials`].
//!
//! See the [Spotify Authorization Guide](https://developer.spotify.com/documentation/web-api/concepts/authorization)
//! for more information on choosing the right authorization flow.

mod client_credentials;
mod pkce;
pub mod scopes;

use crate::{
    RestError,
    api::{ApiError, FormParams, query},
    model::Token,
};
use bytes::Bytes;
pub(crate) use client_credentials::ClientCredentials;
use http::{HeaderMap, HeaderValue, Request, Response as HttpResponse, header, request::Builder};
pub(crate) use pkce::AuthCodePKCE;
use reqwest::blocking::Client;
use thiserror::Error;
use url::Url;

/// A specialized `Result` type for authorization operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Represents errors that can occur during the authorization process.
///
/// This enum defines various error conditions that may arise while handling OAuth
/// authorization flows, such as invalid headers, URL parsing issues, or mismatched
/// state parameters.
///
/// This enum is marked as `#[non_exhaustive]`, meaning new variants may be added in future versions.
/// When matching against it, include a wildcard arm (`_`) to account for any future variants.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// An error related to an invalid HTTP header value.
    ///
    /// # Details
    /// This variant wraps an `http::header::InvalidHeaderValue` error, which occurs
    /// when constructing an HTTP header fails due to invalid characters or formatting.
    #[error("header value error: {0}")]
    HeaderValue(#[from] header::InvalidHeaderValue),

    /// The URL failed to parse.
    ///
    /// # Details
    /// This variant wraps a `url::ParseError`, which occurs when a URL string
    /// cannot be correctly parsed.
    #[error("failed to parse url: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Indicates that the authorization code was not found in the URL.
    ///
    /// This error is returned when the query string of a URL does not contain
    /// the expected `code` parameter during an OAuth callback.
    #[error("authorization code not found")]
    CodeNotFound,

    /// Indicates a mismatch between the expected and received state parameters.
    ///
    /// The state parameter is used in OAuth to prevent CSRF attacks. This error
    /// occurs when the state value returned by the authorization server does not
    /// match the expected value.
    ///
    /// # Fields
    /// - `expected`: The expected state parameter.
    /// - `got`: The state parameter that was received.
    #[error("invalid state parameter: expected {expected} got {got}")]
    InvalidState { expected: String, got: String },

    /// Indicates that the state parameter is missing.
    ///
    /// This error occurs when the state value is expected but not present.
    /// Ensure that the `user_authorization_url()` method is called to generate
    /// a proper authorization URL that includes the state parameter.
    #[error(
        "AuthCodePKCE's state is None. Make sure to generate a user authorization URL by calling user_authorization_url()"
    )]
    NoState,

    /// Indicates that the code verifier is missing.
    ///
    /// This error occurs when the code verifier is expected but not present.
    /// Ensure that the `user_authorization_url()` method is called to generate
    /// the code verifier required for the PKCE flow.
    #[error(
        "AuthCodePKCE's code_verifier is None. Make sure to generate a code verifier by calling user_authorization_url()"
    )]
    NoCodeVerifier,

    /// Represents an error when the access token is empty, indicating that
    /// authentication cannot proceed without a valid token.
    #[error("access token is empty")]
    EmptyAccessToken,

    /// Represents an error when the refresh token is empty, indicating that
    /// token refreshing cannot proceed.
    #[error("refresh token is empty")]
    EmptyRefreshToken,
}

pub(crate) mod private {
    use super::AuthError;
    use crate::{RestError, api::ApiError, model::Token};
    use async_trait::async_trait;
    use reqwest::blocking::Client;

    pub trait AuthFlow {
        fn refresh_token(
            &self,
            client: &Client,
            refresh_token: &str,
        ) -> Result<Token, ApiError<RestError>> {
            let _ = client;
            let _ = refresh_token;
            Err(AuthError::EmptyRefreshToken.into())
        }
    }

    #[async_trait]
    pub trait AsyncAuthFlow {
        async fn refresh_token_async(
            &self,
            client: &reqwest::Client,
            refresh_token: &str,
        ) -> Result<Token, ApiError<RestError>> {
            let _ = client;
            let _ = refresh_token;
            Err(AuthError::EmptyRefreshToken.into())
        }
    }
}

fn request_token(
    client: &Client,
    authorization_header: Option<String>,
    params: FormParams<'_>,
) -> Result<Token, ApiError<RestError>> {
    let (request, data) = init_http_request_and_data(authorization_header, params)?;
    let response = send_http_request(client, request, data).map_err(ApiError::client)?;
    parse_http_response(&response)
}

async fn request_token_async(
    client: &reqwest::Client,
    authorization_header: Option<String>,
    params: FormParams<'_>,
) -> Result<Token, ApiError<RestError>> {
    let (request, data) = init_http_request_and_data(authorization_header, params)?;
    let response = send_http_request_async(client, request, data)
        .await
        .map_err(ApiError::client)?;
    parse_http_response(&response)
}

fn set_authorization_header<'a>(
    headers: &'a mut HeaderMap<HeaderValue>,
    value: &str,
) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
    let mut header_value = HeaderValue::from_str(value)?;
    header_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, header_value);
    Ok(headers)
}

fn init_http_request_and_data(
    authorization_header: Option<String>,
    params: FormParams<'_>,
) -> Result<(Builder, Vec<u8>), ApiError<RestError>> {
    let url = Url::parse("https://accounts.spotify.com/api/token")?;

    let mut req = Request::builder()
        .method(http::Method::POST)
        .uri(query::url_to_http_uri(&url));

    if let Some(value) = authorization_header {
        set_authorization_header(
            req.headers_mut()
                .expect("failed to get headers on the request builder"),
            &value,
        )?;
    }

    let (mime, data) = params
        .into_body()?
        .map_or((None, Vec::new()), |(mime, data)| {
            (Some(mime), data.clone())
        });

    if let Some(mime) = mime {
        req = req.header(header::CONTENT_TYPE, mime);
    }

    req = req.header(header::CONTENT_LENGTH, data.len().to_string());

    Ok((req, data))
}

fn send_http_request(
    client: &Client,
    request: Builder,
    data: Vec<u8>,
) -> Result<http::Response<Bytes>, RestError> {
    let http_request = request.body(data)?;
    let request = http_request.try_into()?;
    let response = client.execute(request)?;

    let mut http_response = HttpResponse::builder()
        .status(response.status())
        .version(response.version());

    let headers = http_response
        .headers_mut()
        .expect("failed to get headers on the request builder");

    for (key, value) in response.headers() {
        headers.insert(key, value.clone());
    }

    Ok(http_response.body(response.bytes()?)?)
}

async fn send_http_request_async(
    client: &reqwest::Client,
    request: Builder,
    data: Vec<u8>,
) -> Result<http::Response<Bytes>, RestError> {
    let http_request = request.body(data)?;
    let request = http_request.try_into()?;
    let response = client.execute(request).await?;

    let mut http_response = HttpResponse::builder()
        .status(response.status())
        .version(response.version());

    let headers = http_response
        .headers_mut()
        .expect("failed to get headers on the request builder");

    for (key, value) in response.headers() {
        headers.insert(key, value.clone());
    }

    Ok(http_response.body(response.bytes().await?)?)
}

fn parse_http_response<T>(response: &http::Response<Bytes>) -> Result<T, ApiError<RestError>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();

    let v = serde_json::from_slice(response.body())
        .map_err(|_e| ApiError::server_error(status, response.body()))?;

    if !status.is_success() {
        return Err(ApiError::from_spotify_with_status(status, v));
    } else if status == http::StatusCode::MOVED_PERMANENTLY {
        return Err(ApiError::moved_permanently(
            response.headers().get(header::LOCATION),
        ));
    }

    serde_json::from_value::<_>(v).map_err(ApiError::data_type::<T>)
}
