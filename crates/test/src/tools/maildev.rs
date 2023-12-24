use anyhow::Result;
use reqwest::{Client, StatusCode};
use scraper::Html;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Recipient {
    pub address: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Header {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub content_type: String,
    pub content_transfer_encoding: String,
    pub date: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EnvelopeRecipient {
    pub address: String,
    pub args: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Envelope {
    pub from: EnvelopeRecipient,
    pub to: Vec<EnvelopeRecipient>,
    pub host: String,
    pub remote_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Mail {
    pub html: String,
    pub headers: Header,
    pub subject: String,
    pub priority: String,
    pub from: Vec<Recipient>,
    pub to: Vec<Recipient>,
    pub date: String,
    pub id: String,
    pub time: String,
    pub read: bool,
    pub envelope: Envelope,
    pub source: String,
    pub size: usize,
    pub size_human: String,
    pub attachments: Option<Vec<String>>,
    pub calculated_bcc: Vec<String>,
}

impl Mail {
    pub fn html(&self) -> Html {
        Html::parse_fragment(&self.html)
    }
}

pub(crate) struct MailDevClient {
    pub client: Client,
}

impl MailDevClient {
    pub(crate) fn new() -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Self { client }
    }

    pub(crate) async fn latest(&self) -> Result<Option<Mail>> {
        let response = self
            .client
            .get("http://localhost:1080/email")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response.json::<Vec<Mail>>().await?;

                if response_body.is_empty() {
                    return Ok(None);
                }

                let mail = response_body.last().unwrap().clone().to_owned();

                Ok(Some(mail))
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => unreachable!(),
        }
    }
}
