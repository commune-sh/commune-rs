use std::net::SocketAddr;

use dotenv::dotenv;
use reqwest::{header::AUTHORIZATION, Client, StatusCode};
use tokio::net::TcpListener;

use commune_server::serve;

pub(crate) struct HttpClient {
    pub client: Client,
    pub addr: SocketAddr,
}

impl HttpClient {
    pub(crate) async fn new() -> Self {
        dotenv().ok();

        let tcp = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = tcp.local_addr().unwrap();

        tokio::spawn(async move {
            serve(tcp).await.expect("Failed to bind to address");
        });

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        HttpClient { client, addr }
    }

    pub(crate) fn get(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.get(self.path(url)),
        }
    }

    pub(crate) fn post(&self, url: &str) -> RequestBuilder {
        RequestBuilder {
            builder: self.client.post(self.path(url)),
        }
    }

    fn path(&self, url: &str) -> String {
        format!("http://{}{}", self.addr, url)
    }
}

pub(crate) struct RequestBuilder {
    builder: reqwest::RequestBuilder,
}

impl RequestBuilder {
    pub(crate) async fn send(self) -> TestResponse {
        TestResponse {
            response: self.builder.send().await.unwrap(),
        }
    }

    pub(crate) fn token(mut self, token: impl AsRef<str>) -> Self {
        let next = self
            .builder
            .header(AUTHORIZATION, format!("Bearer {}", token.as_ref()));

        self.builder = next;
        self
    }

    pub(crate) fn json<T>(mut self, json: &T) -> Self
    where
        T: serde::Serialize,
    {
        self.builder = self.builder.json(json);
        self
    }
}

#[derive(Debug)]
pub(crate) struct TestResponse {
    response: reqwest::Response,
}

impl TestResponse {
    pub(crate) async fn json<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json().await.unwrap()
    }

    pub(crate) fn status(&self) -> StatusCode {
        self.response.status()
    }
}
