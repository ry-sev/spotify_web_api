use crate::api::{ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query, query};
use async_trait::async_trait;
use http::{
    Method, Request,
    header::{self, LOCATION},
};

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
///
/// Use this when you only care about whether the request succeeded,
/// not the response body (e.g., for PUT/DELETE operations).
///
/// # Example
///
/// ```no_run
/// use spotify_web_api::api::{ignore, Query, player::PausePlayback};
///
/// # fn example(client: &impl spotify_web_api::api::Client) {
/// // Pause playback without caring about the response
/// ignore(PausePlayback::default()).query(client).unwrap();
/// # }
/// ```
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;

        self.endpoint.parameters().add_to_url(&mut url);

        let (mime, data) = self
            .endpoint
            .body()?
            .map_or((None, Vec::new()), |(mime, data)| {
                (Some(mime), data.clone())
            });

        let mut req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(&url));

        if let Some(mime) = mime {
            req = req.header(header::CONTENT_TYPE, mime);
        }

        if matches!(self.endpoint.method(), Method::POST | Method::PUT) {
            req = req.header(header::CONTENT_LENGTH, data.len().to_string());
        }

        let rsp = client.rest(req, data)?;
        let status = rsp.status();

        if !status.is_success() {
            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        Ok(())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<(), C> for Ignore<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;

        self.endpoint.parameters().add_to_url(&mut url);

        let (mime, data) = self
            .endpoint
            .body()?
            .map_or((None, Vec::new()), |(mime, data)| {
                (Some(mime), data.clone())
            });

        let mut req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(&url));

        if let Some(mime) = mime {
            req = req.header(header::CONTENT_TYPE, mime);
        }

        if matches!(self.endpoint.method(), Method::POST | Method::PUT) {
            req = req.header(header::CONTENT_LENGTH, data.len().to_string());
        }

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();

        if !status.is_success() {
            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        Ok(())
    }
}
