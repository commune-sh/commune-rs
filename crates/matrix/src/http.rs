use std::str::from_utf8;

use anyhow::{bail, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as HttpClient, Response,
};
use serde::Serialize;
use url::Url;

#[derive(Clone, Debug)]
pub struct Client {
    client: HttpClient,
    base_url: Url,
    token: Option<String>,
    server_name: String,
}

impl Client {
    pub fn new<S: AsRef<str>>(url: S, server_name: S) -> Result<Self> {
        let url = Url::parse(url.as_ref())?;
        let server_name = server_name.as_ref().to_string();

        Ok(Self {
            client: HttpClient::new(),
            base_url: url,
            token: None,
            server_name,
        })
    }

    #[inline]
    pub fn server_name(&self) -> &str {
        &self.server_name
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

    /// Clear the token for safety purposes.
    pub fn clear_token(&mut self) {
        self.token = None;
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

    pub async fn post(&self, path: impl AsRef<str>) -> Result<Response> {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let resp = self.client.post(url).headers(headers).send().await?;

        Ok(resp)
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

    pub async fn delete(&self, path: impl AsRef<str>) -> Result<Response> {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let response = self.client.delete(url).headers(headers).send().await?;

        Ok(response)
    }

    pub async fn delete_json<T>(&self, path: impl AsRef<str>, body: &T) -> Result<Response>
    where
        T: Serialize,
    {
        let url = self.build_url(path)?;
        let headers = self.build_headers()?;
        let resp = self
            .client
            .delete(url)
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
