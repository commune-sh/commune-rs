use std::ops::Deref;
use std::str::from_utf8;

use anyhow::{bail, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client as HttpClient, Response};
use serde::Serialize;
use url::Url;

pub struct Client {
    client: HttpClient,
    base_url: Url,
    token: Option<String>,
}

impl Client {
    pub fn new<S: AsRef<str>>(url: S) -> Result<Self> {
        let url = Url::parse(url.as_ref())?;

        Ok(Self {
            client: HttpClient::new(),
            base_url: url,
            token: None,
        })
    }

    /// Sets the token to be used for authentication with the server.
    pub fn set_token(&mut self, token: impl Into<String>) -> Result<()> {
        let token = token.into();

        if token.is_empty() {
            self.token = None;
            bail!("Token cannot be empty");
        }

        self.token = Some(token);
        Ok(())
    }

    pub async fn get(&self, path: impl AsRef<str>) -> Result<Response> {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let response = self.client.get(url).headers(headers).send().await?;

        Ok(response)
    }

    pub async fn get_query(
        &self,
        path: impl AsRef<str>,
        params: impl Serialize,
    ) -> Result<Response> {
        let url = self.build_url_with_params(path, params)?;
        let headers = self.build_headers()?;
        let response = self.client.get(url).headers(headers).send().await?;

        Ok(response)
    }

    pub async fn post_json<T>(&self, path: impl AsRef<str>, body: &T) -> Result<Response>
    where
        T: Serialize,
    {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let resp = self
            .client
            .post(url)
            .json(body)
            .headers(headers)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn put_json<T>(&self, path: impl AsRef<str>, body: &T) -> Result<Response>
    where
        T: Serialize,
    {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let resp = self
            .client
            .put(url)
            .json(body)
            .headers(headers)
            .send()
            .await?;

        Ok(resp)
    }

    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        if let Some(token) = &self.token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        }

        Ok(headers)
    }

    #[inline]
    fn build_url(&self, path: impl AsRef<str>) -> Result<Url> {
        let mut next = self.base_url.clone();

        next.set_path(path.as_ref());

        Ok(next)
    }

    fn build_url_with_params(&self, path: impl AsRef<str>, params: impl Serialize) -> Result<Url> {
        let mut url = self.build_url(path)?;
        let mut buff = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut buff);

        serde_path_to_error::serialize(&params, qs_ser)?;

        let params = from_utf8(buff.as_slice())?.to_string();

        url.set_query(Some(&params));

        Ok(url)
    }
}

impl Deref for Client {
    type Target = HttpClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
