use crate::{
    api::{self, ApiError, RestClient},
    auth::{
        AuthCodePKCE, AuthError, AuthResult, ClientCredentials,
        private::{AsyncAuthFlow, AuthFlow},
        scopes::Scope,
    },
    model::Token,
};
use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Response as HttpResponse};
use parking_lot::RwLock;
use reqwest::{Client as AsyncClient, blocking::Client};
use std::{collections::HashSet, sync::Arc};
use thiserror::Error;
use url::Url;

const BASE_API_URL: &str = "https://api.spotify.com/v1/";

/// Type alias for a blocking Spotify client using Authorization Code with PKCE flow.
pub type SpotifyPKCE = Spotify<AuthCodePKCE>;

/// Type alias for a blocking Spotify client using Client Credentials flow.
pub type SpotifyClientCredentials = Spotify<ClientCredentials>;

/// Type alias for an async Spotify client using Authorization Code with PKCE flow.
pub type AsyncSpotifyPKCE = AsyncSpotify<AuthCodePKCE>;

/// Type alias for an async Spotify client using Client Credentials flow.
pub type AsyncSpotifyClientCredentials = AsyncSpotify<ClientCredentials>;

/// A specialized `Result` type for Spotify API operations.
pub type SpotifyResult<T> = Result<T, SpotifyError>;

/// Represents errors that can occur during communication with the Spotify API.
///
/// This enum defines various error conditions that may arise while interacting
/// with the Spotify API, such as authentication issues, HTTP errors, or
/// communication failures.
///
/// This enum is marked as `#[non_exhaustive]`, meaning new variants may be added
/// in future versions. When matching against it, include a wildcard arm (`_`)
/// to account for any future variants.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    /// An error related to setting the authentication header.
    ///
    /// This variant wraps an [`AuthError`] that occurs when there is an issue
    /// with the authorization process, such as an invalid state parameter
    /// or a missing authorization code.
    #[error("error setting auth header: {0}")]
    AuthError(#[from] AuthError),

    /// An error during communication with the Spotify API.
    ///
    /// This variant wraps a `reqwest::Error`, which can occur due to network
    /// connectivity issues, timeouts, or unexpected responses from the Spotify API.
    #[error("communication with spotify: {0}")]
    Communication(#[from] reqwest::Error),

    /// An error related to constructing or processing HTTP requests.
    ///
    /// This variant wraps an `http::Error`, which can occur when handling HTTP
    /// requests, such as invalid headers or improperly formed HTTP messages.
    #[error("`http` error: {0}")]
    Http(#[from] http::Error),
}

/// Represents errors that can occur while interacting with the Spotify API.
///
/// This enum captures various error scenarios, including URL parsing failures,
/// authentication issues, communication errors, and API-specific issues.
///
/// This enum is marked as `#[non_exhaustive]`, meaning new variants may be added
/// in future versions. When matching against it, include a wildcard arm (`_`)
/// to account for any future variants.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SpotifyError {
    /// An error that occurs when parsing a URL fails.
    ///
    /// This variant wraps a `url::ParseError`, which indicates an invalid or
    /// malformed URL string.
    #[error("failed to parse url: {0}")]
    UrlParse(#[from] url::ParseError),

    /// An error related to setting the authentication header.
    ///
    /// This variant wraps an `AuthError`, which includes errors such as missing
    /// authorization codes or mismatched state parameters.
    #[error("error setting auth header: {0}")]
    AuthError(#[from] AuthError),

    /// An error during communication with the Spotify API.
    ///
    /// This variant wraps a `reqwest::Error`, representing issues such as network
    /// connectivity failures, timeouts, or unexpected HTTP responses.
    #[error("communication with spotify: {0}")]
    Communication(#[from] reqwest::Error),

    /// An HTTP error returned by the Spotify API.
    ///
    /// This variant indicates a non-2xx HTTP status code was received.
    #[error("spotify HTTP error: {status}")]
    Http {
        /// The HTTP status code returned by the Spotify API.
        status: reqwest::StatusCode,
    },

    /// Indicates that no response was received from the Spotify API.
    ///
    /// This error may occur due to network issues or timeouts.
    #[error("no response from spotify")]
    NoResponse,

    /// An error that occurs when parsing data into a specific data type fails.
    ///
    /// This variant wraps a `serde_json::Error` and includes the name of the type
    /// that failed to parse.
    #[error("could not parse {typename} data: {source}")]
    DataType {
        /// The source of the error.
        source: serde_json::Error,

        /// The name of the type that failed to parse.
        typename: &'static str,
    },

    /// Represents an API error returned by the Spotify API.
    ///
    /// This variant wraps an `ApiError` containing additional details about
    /// the underlying REST error.
    #[error("api error: {0}")]
    Api(#[from] ApiError<RestError>),
}

impl SpotifyError {
    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: std::any::type_name::<T>(),
        }
    }
}

/// A blocking client for interacting with the Spotify Web API.
///
/// This struct provides synchronous methods for making API requests to Spotify.
/// It is parameterized by an authentication flow type `A` which determines how
/// the client authenticates with the Spotify API.
///
/// For most use cases, prefer using the type aliases:
/// - [`SpotifyPKCE`] for user-authorized access (Authorization Code with PKCE)
/// - [`SpotifyClientCredentials`] for app-only access (Client Credentials flow)
///
/// See [`AsyncSpotify`] for an async version of this client.
pub struct Spotify<A>
where
    A: AuthFlow,
{
    /// The client to use for API calls.
    client: Client,

    /// The base URL to use for API calls.
    api_url: Url,

    /// The authentication flow to use for API calls.
    auth: A,

    /// The current access token, if available.
    token: Arc<RwLock<Option<Token>>>,

    /// A handler to call when the access token acquires a new value.
    token_callback: Option<Box<dyn Fn(Token) + 'static>>,
}

impl<A> Spotify<A>
where
    A: AuthFlow,
{
    fn new_impl(auth: A) -> SpotifyResult<Self> {
        let api_url = Url::parse(BASE_API_URL)?;
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        let api = Self {
            client,
            api_url,
            auth,
            token: Arc::new(RwLock::new(None)),
            token_callback: None,
        };
        Ok(api)
    }

    /// Perform a REST query with a given auth.
    fn rest_auth(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<<Self as RestClient>::Error>> {
        let is_expired = self
            .token
            .read()
            .as_ref()
            .ok_or(AuthError::EmptyAccessToken)?
            .is_expired();

        let refresh_token = if is_expired {
            self.token
                .read()
                .as_ref()
                .ok_or(AuthError::EmptyAccessToken)?
                .refresh_token
                .clone()
        } else {
            None
        };

        if let Some(refresh_token) = refresh_token {
            let new_token = self.auth.refresh_token(&self.client, &refresh_token)?;
            self.set_token(new_token);
        }

        let call = || -> Result<_, RestError> {
            self.set_header(
                request
                    .headers_mut()
                    .expect("failed to get headers on the request builder"),
            )?;

            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            let headers = http_rsp
                .headers_mut()
                .expect("failed to get headers on the request builder");

            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(ApiError::client)
    }

    /// Adds the appropriate header to a set of headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value or if the token is empty.
    fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        let token = self.token.read();
        let token = token.as_ref().ok_or(AuthError::EmptyAccessToken)?;

        let value = format!("Bearer {}", token.access_token);
        let mut token_header_value = HeaderValue::from_str(&value).map_err(AuthError::from)?;
        token_header_value.set_sensitive(true);
        headers.insert(http::header::AUTHORIZATION, token_header_value);
        Ok(headers)
    }

    /// Returns a shared reference to the stored access token.
    ///
    /// This method provides access to the current access token stored within the `Spotify` instance.
    /// The access token is required for making authenticated requests to the Spotify Web API.
    ///
    /// The token is wrapped in an `Arc<RwLock<Option<Token>>>` to ensure thread-safe access and
    /// shared ownership. This allows multiple threads to retrieve or update the token as needed.
    ///
    /// # Returns
    /// An `Arc<RwLock<Option<Token>>>` containing:
    /// - `Some(Token)` if an access token is currently available.
    /// - `None` if no access token has been set or retrieved.
    pub fn token(&self) -> Arc<RwLock<Option<Token>>> {
        self.token.clone()
    }

    /// Serializes the currently stored access token to a JSON string.
    ///
    /// This method converts the access token into a `String` in JSON format, allowing
    /// it to be easily stored or transmitted. If no token is stored, it returns `Ok(None)`.
    ///
    /// # Errors
    /// Returns a `SpotifyError::DataType` if serialization of the token fails.
    ///
    /// # Returns
    /// * `Ok(Some(String))` - The serialized token string, if available.
    /// * `Ok(None)` - If no token is currently stored.
    pub fn token_to_string(&self) -> SpotifyResult<Option<String>> {
        let token = self.token.read();
        let Some(token) = token.as_ref() else {
            return Ok(None);
        };
        let s = serde_json::to_string(token).map_err(SpotifyError::data_type::<Token>)?;
        Ok(Some(s))
    }

    fn set_token(&self, mut token: Token) {
        token.expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(token.expires_in as i64));

        if let Some(callback) = &self.token_callback {
            callback(token.clone());
        }

        *self.token.write() = Some(token);
    }
}

impl Spotify<AuthCodePKCE> {
    /// Creates a new instance of `Spotify` configured for the Authorization Code PKCE flow.
    ///
    /// This method initializes the `Spotify` client with an `AuthCodePKCE` authentication method.
    /// The Authorization Code PKCE flow is typically used for client-side applications where user
    /// authentication is required, and it uses a secure code challenge to mitigate interception risks.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `redirect_uri`: The URI to which the user will be redirected after authentication.
    /// - `scopes`: An optional set of scopes that define the permissions the application is requesting.
    ///
    /// # Returns
    /// A [`SpotifyResult`] containing the [`Spotify`] client configured with Authorization Code PKCE authentication,
    /// or a [`SpotifyError`] if initialization fails.
    ///
    /// # Example
    /// ```no_run
    /// use spotify_web_api::{Spotify, auth::scopes};
    ///
    /// let client_id = "your-client-id";
    /// let redirect_uri = "your-redirect-uri";
    ///
    /// let spotify = Spotify::with_authorization_code_pkce(client_id, redirect_uri, scopes::user_details())
    ///     .expect("Failed to create Spotify client");
    /// ```
    pub fn with_authorization_code_pkce(
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        scopes: impl Into<Option<HashSet<Scope>>>,
    ) -> SpotifyResult<Self> {
        let auth = AuthCodePKCE::new(client_id, redirect_uri, scopes);
        Self::new_impl(auth)
    }

    /// Sets the access token for the Spotify client and returns the updated instance.
    ///
    /// This method allows chaining by consuming the current instance, updating the
    /// stored access token, and returning the updated instance.
    ///
    /// The scopes in the token will override the scopes in the `AuthCodePKCE`.
    ///
    /// # Parameters
    /// * `token` - The new access token to be stored in the client.
    ///
    /// # Returns
    /// The updated `Spotify` instance with the new token set.
    pub fn with_token(mut self, token: Token) -> Self {
        let mut scopes = HashSet::new();

        if let Some(scope_str) = &token.scope {
            for s in scope_str.split_whitespace() {
                if let Ok(scope) = Scope::try_from(s) {
                    scopes.insert(scope);
                }
            }
        }
        self.auth.set_scopes(Some(scopes));
        self.token = Arc::new(RwLock::new(Some(token)));
        self
    }

    /// Sets a handler to be called when the access token acquires a new value.
    pub fn token_callback(mut self, handler: impl Fn(Token) + 'static) -> Self {
        self.token_callback = Some(Box::new(handler));
        self
    }

    /// Constructs the full URL for user authorization.
    ///
    /// This method generates the state and code verifier parameters to produce the complete
    /// authorization URL. The user should be redirected to this URL to begin the authorization
    /// process.
    ///
    /// # Returns
    /// * `String` - The fully constructed authorization URL.
    pub fn user_authorization_url(&mut self) -> String {
        self.auth.user_authorization_url()
    }

    /// Verifies the authorization code and state returned in the callback URL.
    ///
    /// This method extracts the `code` and `state` parameters from the provided URL. It ensures
    /// that the `state` matches the one generated earlier, rejecting the response if there is a
    /// mismatch or if the required parameters are missing.
    ///
    /// # Arguments
    /// * `url` - A string slice containing the callback URL provided by the OAuth provider.
    ///
    /// # Returns
    /// * `Ok(String)` containing the authorization code if verification succeeds.
    /// * `Err(AuthError)` if the `code` or `state` is missing, or if the `state` does not match.
    ///
    /// # Errors
    /// * `AuthError::NoState` - Returned if the `self.state` is None.
    /// * `AuthError::CodeNotFound` - Returned if the `code` parameter is missing in the URL.
    /// * `AuthError::InvalidState` - Returned if the `state` parameter is missing or does not match
    ///   the expected value.
    pub fn verify_authorization_code(&self, url: &str) -> AuthResult<String> {
        self.auth.verify_authorization_code(url)
    }

    /// Requests an access token using the provided authorization code.
    ///
    /// This method exchanges the authorization code obtained from the callback URL for an access token.
    /// The access token is required to authenticate API requests. The obtained token is stored
    /// internally and is valid for the duration specified by Spotify.
    ///
    /// # Arguments
    /// * `code` - A string slice containing the authorization code provided by Spotify.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully retrieved and stored.
    /// * `Err(ApiError<RestError>)` - If the token request fails due to network issues, invalid authorization code, or other API errors.
    pub fn request_token(&self, code: &str) -> Result<(), ApiError<RestError>> {
        let token = self.auth.request_token(code, &self.client)?;
        self.set_token(token);
        Ok(())
    }

    /// Requests an access token using the provided redirect URL.
    ///
    /// This method extracts the authorization code from the callback URL and exchanges it for an
    /// access token. It combines the `verify_authorization_code` and `request_token` methods
    /// for convenience, handling both verification and token retrieval in a single call.
    ///
    /// # Arguments
    /// * `url` - A string slice containing the callback URL redirected to by Spotify after user authorization.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully retrieved and stored.
    /// * `Err(ApiError<RestError>)` - If the token request fails due to network issues, invalid authorization code, or other API errors.
    pub fn request_token_from_redirect_url(&self, url: &str) -> Result<(), ApiError<RestError>> {
        let token = self
            .auth
            .request_token_from_redirect_url(url, &self.client)?;
        self.set_token(token);
        Ok(())
    }

    /// Refreshes the access token using the stored refresh token.
    ///
    /// This method retrieves a new access token by exchanging the stored refresh token.
    /// It requires that a valid refresh token is present in the current token.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully refreshed and updated.
    /// * `Err(AuthError::EmptyAccessToken)` - If no token is available.
    /// * `Err(AuthError::EmptyRefreshToken)` - If no refresh token is available.
    /// * `Err(ApiError<RestError>)` - If the token refresh request fails due to network issues
    ///   or other API errors.
    pub fn refresh_token(&self) -> Result<(), ApiError<RestError>> {
        let refresh_token = self
            .token
            .read()
            .as_ref()
            .ok_or(AuthError::EmptyAccessToken)?
            .refresh_token
            .clone()
            .ok_or(AuthError::EmptyRefreshToken)?;

        let token = self.auth.refresh_token(&self.client, &refresh_token)?;
        self.set_token(token);

        Ok(())
    }
}

impl Spotify<ClientCredentials> {
    /// Creates a new instance of `Spotify` configured for the Client Credentials flow.
    ///
    /// This method initializes the `Spotify` client with a `ClientCredentials` authentication method.
    /// The Client Credentials flow is typically used for server-to-server interactions where user
    /// authentication is not required, such as accessing public Spotify API endpoints.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `client_secret`: The Client Secret of your Spotify application.
    ///
    /// # Returns
    /// A [`SpotifyResult`] containing the [`Spotify`] client configured with Client Credentials authentication,
    /// or a [`SpotifyError`] if initialization fails.
    ///
    /// # Example
    /// ```no_run
    /// use spotify_web_api::Spotify;
    ///
    /// let client_id = "your-client-id";
    /// let client_secret = "your-client-secret";
    ///
    /// let spotify = Spotify::with_client_credentials(client_id, client_secret)
    ///     .expect("Failed to create Spotify client");
    /// ```
    pub fn with_client_credentials(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> SpotifyResult<Self> {
        let auth = ClientCredentials::new(client_id, client_secret);
        Self::new_impl(auth)
    }

    /// Sets the access token for the Spotify client and returns the updated instance.
    ///
    /// This method allows chaining by consuming the current instance, updating the
    /// stored access token, and returning the updated instance.
    ///
    /// The `refresh_token` and `scope` fields will be set to `None` in the token.
    ///
    /// # Parameters
    /// * `token` - The new access token to be stored in the client.
    ///
    /// # Returns
    /// The updated `Spotify` instance with the new token set.
    ///
    /// # Note:
    /// Once the token is expired, subsequent requests will fail.
    pub fn with_token(mut self, mut token: Token) -> Self {
        token.refresh_token = None;
        token.scope = None;
        self.token = Arc::new(RwLock::new(Some(token)));
        self
    }

    /// Requests an access token using the configured Client Credentials flow.
    ///
    /// This method sends a request to the Spotify authorization server to obtain an access token.
    /// The access token is required to authenticate API requests. The obtained token is stored
    /// internally and is valid for the duration specified by Spotify.
    ///
    /// # Returns
    /// - `Ok(())`: If the token was successfully retrieved and stored.
    /// - `Err(ApiError<RestError>)`: If the token request fails due to network issues, invalid credentials, or other API errors.
    ///
    /// # Example
    /// ```no_run
    /// use spotify_web_api::Spotify;
    ///
    /// let client_id = "your-client-id";
    /// let client_secret = "your-client-secret";
    ///
    /// let mut spotify = Spotify::with_client_credentials(client_id, client_secret)
    ///     .expect("Failed to create Spotify client");
    ///
    /// spotify.request_token().expect("Failed to request token");
    /// ```
    pub fn request_token(&self) -> Result<(), ApiError<RestError>> {
        let token = self.auth.request_token(&self.client)?;
        self.set_token(token);
        Ok(())
    }
}

impl<A> RestClient for Spotify<A>
where
    A: AuthFlow,
{
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        log::info!("REST api call {endpoint}");
        Ok(self.api_url.join(endpoint)?)
    }
}

impl<A> api::Client for Spotify<A>
where
    A: AuthFlow,
{
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        self.rest_auth(request, body)
    }
}

/// An async client for interacting with the Spotify Web API.
///
/// This struct provides asynchronous methods for making API requests to Spotify.
/// It is parameterized by an authentication flow type `A` which determines how
/// the client authenticates with the Spotify API.
///
/// For most use cases, prefer using the type aliases:
/// - [`AsyncSpotifyPKCE`] for user-authorized access (Authorization Code with PKCE)
/// - [`AsyncSpotifyClientCredentials`] for app-only access (Client Credentials flow)
///
/// See [`Spotify`] for a blocking version of this client.
pub struct AsyncSpotify<A>
where
    A: AsyncAuthFlow,
{
    /// The client to use for API calls.
    client: reqwest::Client,

    /// The base URL to use for API calls.
    api_url: Url,

    /// The authentication flow to use for API calls.
    auth: A,

    /// The current access token, if available.
    token: Arc<RwLock<Option<Token>>>,

    /// A handler to call when the access token acquires a new value.
    token_callback: Option<Box<dyn Fn(Token) + Send + Sync + 'static>>,
}

impl<A> AsyncSpotify<A>
where
    A: AsyncAuthFlow + Sync,
{
    fn new_impl(auth: A) -> SpotifyResult<Self> {
        let api_url = Url::parse(BASE_API_URL)?;
        let client = AsyncClient::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        let api = Self {
            client,
            api_url,
            auth,
            token: Arc::new(RwLock::new(None)),
            token_callback: None,
        };
        Ok(api)
    }

    /// Perform a REST query with a given auth.
    async fn rest_async_auth(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<<Self as RestClient>::Error>> {
        use futures_util::TryFutureExt;

        let is_expired = self
            .token
            .read()
            .as_ref()
            .ok_or(AuthError::EmptyAccessToken)?
            .is_expired();

        let refresh_token = if is_expired {
            self.token
                .read()
                .as_ref()
                .ok_or(AuthError::EmptyAccessToken)?
                .refresh_token
                .clone()
        } else {
            None
        };

        if let Some(refresh_token) = refresh_token {
            let new_token = self
                .auth
                .refresh_token_async(&self.client, &refresh_token)
                .await?;

            self.set_token(new_token);
        }

        let call = || async {
            self.set_header(
                request
                    .headers_mut()
                    .expect("failed to get headers on the request builder"),
            )?;

            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            let headers = http_rsp
                .headers_mut()
                .expect("failed to get headers on the request builder");

            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call().map_err(ApiError::client).await
    }

    /// Adds the appropriate header to a set of headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value or if the token is empty.
    fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        let token = self.token.read();
        let token = token.as_ref().ok_or(AuthError::EmptyAccessToken)?;

        let value = format!("Bearer {}", token.access_token);
        let mut token_header_value = HeaderValue::from_str(&value).map_err(AuthError::from)?;
        token_header_value.set_sensitive(true);
        headers.insert(http::header::AUTHORIZATION, token_header_value);
        Ok(headers)
    }

    /// Returns a shared reference to the stored access token.
    ///
    /// This method provides access to the current access token stored within the `Spotify` instance.
    /// The access token is required for making authenticated requests to the Spotify Web API.
    ///
    /// The token is wrapped in an `Arc<RwLock<Option<Token>>>` to ensure thread-safe access and
    /// shared ownership. This allows multiple threads to retrieve or update the token as needed.
    ///
    /// # Returns
    /// An `Arc<RwLock<Option<Token>>>` containing:
    /// - `Some(Token)` if an access token is currently available.
    /// - `None` if no access token has been set or retrieved.
    pub fn token(&self) -> Arc<RwLock<Option<Token>>> {
        self.token.clone()
    }

    /// Serializes the currently stored access token to a JSON string.
    ///
    /// This method converts the access token into a `String` in JSON format, allowing
    /// it to be easily stored or transmitted. If no token is stored, it returns `Ok(None)`.
    ///
    /// # Errors
    /// Returns a `SpotifyError::DataType` if serialization of the token fails.
    ///
    /// # Returns
    /// * `Ok(Some(String))` - The serialized token string, if available.
    /// * `Ok(None)` - If no token is currently stored.
    pub fn token_to_string(&self) -> SpotifyResult<Option<String>> {
        let token = self.token.read();
        let Some(token) = token.as_ref() else {
            return Ok(None);
        };
        let s = serde_json::to_string(token).map_err(SpotifyError::data_type::<Token>)?;
        Ok(Some(s))
    }

    fn set_token(&self, mut token: Token) {
        token.expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(token.expires_in as i64));

        if let Some(callback) = &self.token_callback {
            callback(token.clone());
        }

        *self.token.write() = Some(token);
    }
}

impl AsyncSpotify<AuthCodePKCE> {
    /// Creates a new instance of `Spotify` configured for the Authorization Code PKCE flow.
    ///
    /// This method initializes the `Spotify` client with an `AuthCodePKCE` authentication method.
    /// The Authorization Code PKCE flow is typically used for client-side applications where user
    /// authentication is required, and it uses a secure code challenge to mitigate interception risks.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `redirect_uri`: The URI to which the user will be redirected after authentication.
    /// - `scopes`: An optional set of scopes that define the permissions the application is requesting.
    ///
    /// # Returns
    /// A [`SpotifyResult`] containing the [`Spotify`] client configured with Authorization Code PKCE authentication,
    /// or a [`SpotifyError`] if initialization fails.
    ///
    /// # Example
    /// ```no_run
    /// use spotify_web_api::{AsyncSpotify, auth::scopes};
    ///
    /// let client_id = "your-client-id";
    /// let redirect_uri = "your-redirect-uri";
    ///
    /// let spotify = AsyncSpotify::with_authorization_code_pkce(client_id, redirect_uri, scopes::user_details())
    ///     .expect("Failed to create Spotify client");
    /// ```
    pub fn with_authorization_code_pkce(
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        scopes: impl Into<Option<HashSet<Scope>>>,
    ) -> SpotifyResult<Self> {
        let auth = AuthCodePKCE::new(client_id, redirect_uri, scopes);
        Self::new_impl(auth)
    }

    /// Sets the access token for the Spotify client and returns the updated instance.
    ///
    /// This method allows chaining by consuming the current instance, updating the
    /// stored access token, and returning the updated instance.
    ///
    /// The scopes in the token will override the scopes in the `AuthCodePKCE`.
    ///
    /// # Parameters
    /// * `token` - The new access token to be stored in the client.
    ///
    /// # Returns
    /// The updated `Spotify` instance with the new token set.
    pub fn with_token(mut self, token: Token) -> Self {
        let mut scopes = HashSet::new();
        if let Some(scope_str) = &token.scope {
            for s in scope_str.split_whitespace() {
                if let Ok(scope) = Scope::try_from(s) {
                    scopes.insert(scope);
                }
            }
        }
        self.auth.set_scopes(Some(scopes));
        self.token = Arc::new(RwLock::new(Some(token)));
        self
    }

    /// Sets a handler to be called when the access token acquires a new value.
    pub fn token_callback(mut self, handler: impl Fn(Token) + Send + Sync + 'static) -> Self {
        self.token_callback = Some(Box::new(handler));
        self
    }

    /// Constructs the full URL for user authorization.
    ///
    /// This method generates the state and code verifier parameters to produce the complete
    /// authorization URL. The user should be redirected to this URL to begin the authorization
    /// process.
    ///
    /// # Returns
    /// * `String` - The fully constructed authorization URL.
    pub fn user_authorization_url(&mut self) -> String {
        self.auth.user_authorization_url()
    }

    /// Verifies the authorization code and state returned in the callback URL.
    ///
    /// This method extracts the `code` and `state` parameters from the provided URL. It ensures
    /// that the `state` matches the one generated earlier, rejecting the response if there is a
    /// mismatch or if the required parameters are missing.
    ///
    /// # Arguments
    /// * `url` - A string slice containing the callback URL provided by the OAuth provider.
    ///
    /// # Returns
    /// * `Ok(String)` containing the authorization code if verification succeeds.
    /// * `Err(AuthError)` if the `code` or `state` is missing, or if the `state` does not match.
    ///
    /// # Errors
    /// * `AuthError::NoState` - Returned if the `self.state` is None.
    /// * `AuthError::CodeNotFound` - Returned if the `code` parameter is missing in the URL.
    /// * `AuthError::InvalidState` - Returned if the `state` parameter is missing or does not match
    ///   the expected value.
    pub fn verify_authorization_code(&self, url: &str) -> AuthResult<String> {
        self.auth.verify_authorization_code(url)
    }

    /// Asynchronously requests an access token using the provided authorization code.
    ///
    /// This method exchanges the authorization code obtained from the callback URL for an access token.
    /// The access token is required to authenticate API requests. The obtained token is stored
    /// internally and is valid for the duration specified by Spotify.
    ///
    /// # Arguments
    /// * `code` - A string slice containing the authorization code provided by Spotify.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully retrieved and stored.
    /// * `Err(ApiError<RestError>)` - If the token request fails due to network issues, invalid authorization code, or other API errors.
    pub async fn request_token(&self, code: &str) -> Result<(), ApiError<RestError>> {
        let token = self.auth.request_token_async(code, &self.client).await?;
        self.set_token(token);
        Ok(())
    }

    /// Asynchronously requests an access token using the provided redirect URL.
    ///
    /// This method extracts the authorization code from the callback URL and exchanges it for an
    /// access token. It combines the `verify_authorization_code` and `request_token` methods
    /// for convenience, handling both verification and token retrieval in a single call.
    ///
    /// # Arguments
    /// * `url` - A string slice containing the callback URL redirected to by Spotify after user authorization.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully retrieved and stored.
    /// * `Err(ApiError<RestError>)` - If the token request fails due to network issues, invalid authorization code, or other API errors.
    pub async fn request_token_from_redirect_url(
        &self,
        url: &str,
    ) -> Result<(), ApiError<RestError>> {
        let token = self
            .auth
            .request_token_from_redirect_url_async(url, &self.client)
            .await?;
        self.set_token(token);
        Ok(())
    }

    /// Asynchronously refreshes the access token using the stored refresh token.
    ///
    /// This method retrieves a new access token by exchanging the stored refresh token.
    /// It requires that a valid refresh token is present in the current token.
    ///
    /// # Returns
    /// * `Ok(())` - If the token was successfully refreshed and updated.
    /// * `Err(AuthError::EmptyAccessToken)` - If no token is available.
    /// * `Err(AuthError::EmptyRefreshToken)` - If no refresh token is available.
    /// * `Err(ApiError<RestError>)` - If the token refresh request fails due to network issues
    ///   or other API errors.
    pub async fn refresh_token(&self) -> Result<(), ApiError<RestError>> {
        let refresh_token = self
            .token
            .read()
            .as_ref()
            .ok_or(AuthError::EmptyAccessToken)?
            .refresh_token
            .clone()
            .ok_or(AuthError::EmptyRefreshToken)?;

        let token = self
            .auth
            .refresh_token_async(&self.client, &refresh_token)
            .await?;

        self.set_token(token);

        Ok(())
    }
}

impl AsyncSpotify<ClientCredentials> {
    /// Creates a new instance of `Spotify` configured for the Client Credentials flow.
    ///
    /// This method initializes the `Spotify` client with a `ClientCredentials` authentication method.
    /// The Client Credentials flow is typically used for server-to-server interactions where user
    /// authentication is not required, such as accessing public Spotify API endpoints.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `client_secret`: The Client Secret of your Spotify application.
    ///
    /// # Returns
    /// A [`SpotifyResult`] containing the [`Spotify`] client configured with Client Credentials authentication,
    /// or a [`SpotifyError`] if initialization fails.
    ///
    /// # Example
    /// ```no_run
    /// use spotify_web_api::AsyncSpotify;
    ///
    /// let client_id = "your-client-id";
    /// let client_secret = "your-client-secret";
    ///
    /// let spotify = AsyncSpotify::with_client_credentials(client_id, client_secret)
    ///     .expect("Failed to create Spotify client");
    /// ```
    pub fn with_client_credentials(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> SpotifyResult<Self> {
        let auth = ClientCredentials::new(client_id, client_secret);
        Self::new_impl(auth)
    }

    /// Sets the access token for the Spotify client and returns the updated instance.
    ///
    /// This method allows chaining by consuming the current instance, updating the
    /// stored access token, and returning the updated instance.
    ///
    /// The `refresh_token`, and `scope` fields will be set to `None` in the token.
    ///
    /// # Parameters
    /// * `token` - The new access token to be stored in the client.
    ///
    /// # Returns
    /// The updated `Spotify` instance with the new token set.
    ///
    /// # Note:
    /// Once the token is expired, subsequent requests will fail.
    pub fn with_token(mut self, mut token: Token) -> Self {
        token.refresh_token = None;
        token.scope = None;
        self.token = Arc::new(RwLock::new(Some(token)));
        self
    }

    /// Asynchronously requests an access token using the configured Client Credentials flow.
    ///
    /// This method sends a request to the Spotify authorization server to obtain an access token.
    /// The access token is required to authenticate API requests. The obtained token is stored
    /// internally and is valid for the duration specified by Spotify.
    ///
    /// # Returns
    /// - `Ok(())`: If the token was successfully retrieved and stored.
    /// - `Err(ApiError<RestError>)`: If the token request fails due to network issues, invalid credentials, or other API errors.
    pub async fn request_token(&self) -> Result<(), ApiError<RestError>> {
        let token = self.auth.request_token_async(&self.client).await?;
        self.set_token(token);
        Ok(())
    }
}

#[async_trait]
impl<A> RestClient for AsyncSpotify<A>
where
    A: AsyncAuthFlow,
{
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        log::info!("REST api call {endpoint}");
        Ok(self.api_url.join(endpoint)?)
    }
}

#[async_trait]
impl<A> api::AsyncClient for AsyncSpotify<A>
where
    A: AsyncAuthFlow + Sync + Send,
{
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        self.rest_async_auth(request, body).await
    }
}
