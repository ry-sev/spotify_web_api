use super::{MAX_LIMIT, Pageable, Pagination};
use crate::{
    api::{ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query, query},
    model::Page,
};
use async_trait::async_trait;
use http::{Method, Request, header};
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

/// A query modifier that paginates an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(crate) endpoint: E,
    pub(crate) pagination: Pagination,
}

/// Collect data from a paged endpoint.
///
/// Use this for fine-grained control over pagination behavior.
/// For common use cases, prefer [`paged_all`] or [`paged_with_limit`].
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

/// Collect all data from a paged endpoint.
///
/// This will make multiple API requests as needed to fetch all available items.
///
/// # Example
///
/// ```no_run
/// use spotify_web_api::api::{paged_all, Query, playlists::GetCurrentUserPlaylists};
/// use spotify_web_api::model::SimplifiedPlaylist;
///
/// # fn example(client: &impl spotify_web_api::api::Client) {
/// // Get all of the current user's playlists
/// let playlists: Vec<SimplifiedPlaylist> = paged_all(GetCurrentUserPlaylists::default())
///     .query(client)
///     .unwrap();
/// # }
/// ```
pub fn paged_all<E>(endpoint: E) -> Paged<E> {
    paged(endpoint, Pagination::All)
}

/// Collect a limited amount of data from a paged endpoint.
///
/// If the limit is greater than the maximum limit of 50, the maximum limit will be used.
pub fn paged_with_limit<E>(endpoint: E, limit: usize) -> Paged<E> {
    paged(endpoint, Pagination::Limit(limit.min(MAX_LIMIT)))
}

/// Collect a limited amount of data from a paged endpoint starting at an offset.
///
/// If the limit is greater than the maximum limit of 50, the maximum limit will be used.
pub fn paged_with_limit_and_offset<E>(endpoint: E, limit: usize, offset: usize) -> Paged<E> {
    paged(
        endpoint,
        Pagination::Page {
            limit: limit.min(MAX_LIMIT),
            offset,
        },
    )
}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned + 'static,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        self.iter(client).collect()
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = {
            let mut url = self
                .endpoint
                .url_base()
                .endpoint_for(client, &self.endpoint.endpoint())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        let results = Arc::new(Mutex::new(Vec::new()));
        let body = self.endpoint.body()?;
        let mut next_url = None;

        let offset = match self.pagination {
            Pagination::Page { offset, .. } => offset,
            _ => 0,
        };

        loop {
            let page_url = next_url.take().unwrap_or_else(|| {
                let mut page_url = url.clone();
                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("offset", &offset.to_string());
                    pairs.append_pair("limit", &self.pagination.limit().to_string());
                }
                page_url
            });

            let (mime, data) = body.as_ref().map_or((None, Vec::new()), |(mime, data)| {
                (Some(mime), data.clone())
            });

            let mut req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(&page_url));

            if let Some(mime) = mime {
                req = req.header(header::CONTENT_TYPE, *mime);
            }

            if matches!(self.endpoint.method(), Method::POST | Method::PUT) {
                req = req.header(header::CONTENT_LENGTH, data.len().to_string());
            }

            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;

            if !status.is_success() {
                return Err(ApiError::from_spotify_with_status(status, v));
            } else if status == http::StatusCode::MOVED_PERMANENTLY {
                return Err(ApiError::moved_permanently(
                    rsp.headers().get(header::LOCATION),
                ));
            }

            let page: Page<T> =
                serde_json::from_value(v).map_err(ApiError::data_type::<Page<T>>)?;

            let page_len = page.items.len();
            next_url = page.next.as_ref().map(|url| Url::parse(url)).transpose()?;

            let mut locked_results = results.lock();
            locked_results.extend(page.items);

            if self.pagination.is_last_page(page_len, locked_results.len()) || next_url.is_none() {
                break;
            }
        }

        let mut locked_results = results.lock();

        Ok(std::mem::take(&mut locked_results))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::client::{ExpectedUrl, PagedTestClient};
    use http::Method;
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;

    use super::*;

    #[derive(Debug, Default)]
    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }
    }

    impl Pageable for Dummy {}

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[tokio::test]
    async fn pagination_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = paged(Dummy, Pagination::Limit(3))
            .query_async(&client)
            .await
            .unwrap();

        assert_eq!(res.len(), 3);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn pagination_invalid_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = paged(Dummy, Pagination::Limit(100))
            .query_async(&client)
            .await
            .unwrap();

        assert_eq!(res.len(), 50);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn pagination_limit_and_offset_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = paged_with_limit_and_offset(Dummy, 5, 15)
            .query_async(&client)
            .await
            .unwrap();

        assert_eq!(res.len(), 5);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, (i + 15) as u8);
        }
    }

    #[tokio::test]
    async fn pagination_all_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=55).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = paged(Dummy, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();

        assert_eq!(res.len(), 56);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }
}
