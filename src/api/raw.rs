use crate::api::{ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query, query};
use async_trait::async_trait;
use http::{Method, Request, header};

/// A query modifier that returns the raw data from the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<E> {
    endpoint: E,
}

/// Return the raw data from the endpoint.
///
/// Use this when you need the raw bytes of the response instead of
/// a deserialized type.
///
/// # Example
///
/// ```no_run
/// use spotify_web_api::api::{raw, Query, albums::GetAlbum};
///
/// # fn example(client: &impl spotify_web_api::api::Client) {
/// // Get raw JSON bytes
/// let bytes: Vec<u8> = raw(GetAlbum::from("album_id")).query(client).unwrap();
/// # }
/// ```
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw { endpoint }
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(&url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        if !status.is_success() {
            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                rsp.headers().get(header::LOCATION),
            ));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<Vec<u8>, C> for Raw<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
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
            return Err(ApiError::moved_permanently(
                rsp.headers().get(header::LOCATION),
            ));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{self, ApiError, AsyncQuery, Endpoint, Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };
    use http::{Method, StatusCode};
    use serde_json::json;
    use std::borrow::Cow;

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[test]
    fn test_spotify_non_json_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let data = api::raw(Dummy).query(&client).unwrap();
        itertools::assert_equal(data, "not json".bytes());
    }

    #[tokio::test]
    async fn test_spotify_non_json_response_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let data = api::raw(Dummy).query_async(&client).await.unwrap();
        itertools::assert_equal(data, "not json".bytes());
    }

    #[test]
    fn test_spotify_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");

        let err = api::raw(Dummy).query(&client).unwrap_err();
        if let ApiError::SpotifyService { status, .. } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[test]
    fn test_spotify_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let err = api::raw(Dummy).query(&client).unwrap_err();
        if let ApiError::SpotifyWithStatus { status, msg } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[test]
    fn test_spotify_error_detection_legacy() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );

        let err = api::raw(Dummy).query(&client).unwrap_err();
        if let ApiError::SpotifyWithStatus { status, msg } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[test]
    fn test_spotify_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let err = api::raw(Dummy).query(&client).unwrap_err();
        if let ApiError::SpotifyUnrecognizedWithStatus { status, obj } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {err}");
        }
    }
}
