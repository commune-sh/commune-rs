use anyhow::Result;
use ruma_common::{MxcUri, OwnedMxcUri};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use chrono::{serde::ts_microseconds_option, DateTime, Utc};

use crate::error::MatrixError;

#[derive(Debug, Serialize)]
pub struct GetPreviewUrlQuery {
    pub url: url::Url,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMxcUriResponse {
    pub content_uri: String,

    #[serde(with = "ts_microseconds_option")]
    pub unused_expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct GetPreviewUrlResponse {
    #[serde(rename = "matrix:image_size")]
    pub image_size: Option<u64>,

    #[serde(rename = "og:description")]
    pub description: Option<String>,

    #[serde(rename = "og:image")]
    pub image: Option<OwnedMxcUri>,

    #[serde(rename = "og:image:height")]
    pub height: Option<u64>,

    #[serde(rename = "og:image:width")]
    pub width: Option<u64>,

    #[serde(rename = "og:image:type")]
    pub kind: Option<mime::Mime>,

    #[serde(rename = "og:title")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetConfigResponse {
    #[serde(rename = "m.upload.size")]
    pub upload_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub enum ResizeMethod {
    Crop,
    Scale,
}

pub struct MxcService;

#[derive(Debug, Deserialize)]
pub struct MxcError {
    #[serde(flatten)]
    pub inner: MatrixError,

    pub retry_after_ms: u64,
}

impl MxcService {
    /// Creates a new `MxcUri`, independently of the content being uploaded
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#post_matrixmediav1create
    #[instrument(skip(client, access_token))]
    pub async fn create(
        client: &crate::http::Client,
        access_token: impl Into<String>,
    ) -> Result<CreateMxcUriResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp.post("/_matrix/media/v1/create").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MxcError>().await?;

        Err(anyhow::anyhow!(error.inner.error))
    }

    /// Retrieve the configuration of the content repository
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#get_matrixmediav3config
    #[instrument(skip(client, access_token))]
    pub async fn get_config(
        client: &crate::http::Client,
        access_token: impl Into<String>,
    ) -> Result<GetConfigResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp.get("/_matrix/media/v3/config").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MxcError>().await?;

        Err(anyhow::anyhow!(error.inner.error))
    }

    /// Retrieve a URL to download content from the content repository, optionally replacing the
    /// name of the file.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#get_matrixmediav3downloadservernamemediaid
    #[instrument(skip(client, access_token))]
    pub async fn get_download_url(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        mxc_uri: &MxcUri,
        mut base_url: url::Url,
        file_name: Option<String>,
    ) -> Result<url::Url> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let (server_name, media_id) = mxc_uri.parts().unwrap();

        let mut path = format!(
            "/_matrix/media/v3/download/{server_name}/{media_id}",
            server_name = server_name,
            media_id = media_id,
        );

        if let Some(file_name) = file_name {
            path.push_str(&format!("/{file_name}", file_name = file_name))
        }

        base_url.set_path(&path);

        Ok(base_url)
    }

    /// 
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#get_matrixmediav3preview_url
    #[instrument(skip(client, access_token))]
    pub async fn get_preview(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        query: GetPreviewUrlQuery,
    ) -> Result<GetPreviewUrlResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .get_query(format!("/_matrix/media/v3/preview_url",), &query)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MxcError>().await?;

        Err(anyhow::anyhow!(error.inner.error))
    }
}
