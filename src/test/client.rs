use crate::api::{ApiError, AsyncClient, Client, RestClient};
use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use http::{header, request::Builder as RequestBuilder, Method, Response, StatusCode};
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
use thiserror::Error;
use url::Url;

#[derive(Debug, Builder)]
pub struct ExpectedUrl {
    #[builder(default = "Method::GET")]
    pub method: Method,

    pub endpoint: &'static str,

    #[builder(default)]
    pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    #[builder(setter(strip_option, into), default)]
    pub content_type: Option<String>,

    #[builder(default)]
    pub body: Vec<u8>,

    #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,
}

impl ExpectedUrlBuilder {
    pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
        self.query
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().copied().map(|(k, v)| (k.into(), v.into())));
        self
    }

    #[allow(dead_code)]
    pub fn body_str(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.bytes().collect());
        self
    }
}

impl ExpectedUrl {
    pub fn builder() -> ExpectedUrlBuilder {
        ExpectedUrlBuilder::default()
    }

    fn check(&self, method: &Method, url: &Url) {
        // Test that the method is as expected.
        assert_eq!(method, self.method);

        // Ensure that the URL was not tampered with in the meantime.
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
        assert_eq!(url.host_str().unwrap(), "api.spotify.com");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), format!("/v1/{}", self.endpoint));

        let mut count = 0;

        url.query_pairs().into_iter().for_each(|(key, value)| {
            let found = self.query.iter().any(|(expected_key, expected_value)| {
                &key == expected_key && &value == expected_value
            });
            assert!(found, "unexpected query parameter `{key}={value}`");
            count += 1;
        });

        assert_eq!(count, self.query.len());
        assert_eq!(url.fragment(), None);
    }
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    data: Vec<u8>,
}

impl MockResponse {
    fn response(&self) -> Response<Vec<u8>> {
        Response::builder()
            .status(self.status)
            .body(self.data.clone())
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct MockClient {
    response_map: HashMap<(Method, String), MockResponse>,
}

pub struct SingleTestClient {
    client: MockClient,
    expected: ExpectedUrl,
}

impl SingleTestClient {
    pub fn new_raw<T>(expected: ExpectedUrl, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        let mut client = MockClient::default();

        let request = (
            expected.method.clone(),
            format!("/v1/{}", expected.endpoint),
        );
        let response = MockResponse {
            status: expected.status,
            data: data.into(),
        };

        client.response_map.insert(request, response);

        Self { client, expected }
    }

    #[allow(dead_code)]
    pub fn new_json<T>(expected: ExpectedUrl, data: &T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(data).unwrap();
        Self::new_raw(expected, data)
    }
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for SingleTestClient {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!(
            "https://api.spotify.com/v1/{endpoint}"
        ))?)
    }
}

impl Client for SingleTestClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();
        self.expected
            .check(&request.method_ref().unwrap().clone(), &url);

        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );

        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());

        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(
                content_type,
                std::iter::once(&expected_content_type).copied(),
            );
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let request = request.body(body).unwrap();

        Ok(self
            .client
            .response_map
            .get(&(request.method().clone(), request.uri().path().into()))
            .expect("no matching request found")
            .response()
            .map(Into::into))
    }
}

#[async_trait]
impl AsyncClient for SingleTestClient {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}
