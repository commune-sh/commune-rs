use std::net::{SocketAddr, TcpListener};

use dotenv::dotenv;
use reqwest::{Client, StatusCode};

use commune_server::serve;

pub(crate) struct HttpClient {
    pub client: Client,
    pub addr: SocketAddr,
}

impl HttpClient {
    pub(crate) async fn new() -> Self {
        dotenv().ok();

        let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
        let tcp = TcpListener::bind(addr).unwrap();

        tokio::spawn(async move {
            serve(tcp).await.expect("Failed to bind to address");
        });

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        HttpClient { client, addr }
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
