use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use url::Url;

use crate::mail::error::MailErrorCode;
use crate::{CommuneConfig, Result};

pub struct MailDevConfig {
    pub smtp_host: Url,
}

pub enum EmailProvider {
    MailDev(MailDevConfig),
}

impl EmailProvider {
    pub fn new(config: &CommuneConfig) -> Self {
        if let Some(smtp_host) = &config.smtp_host {
            tracing::warn!(
                %smtp_host,
                "Using MailDev as email provider! This is only for development!"
            );

            return EmailProvider::MailDev(MailDevConfig {
                smtp_host: smtp_host.to_owned(),
            });
        }

        panic!("No email provider configured");
    }

    pub fn send_mail(&self, from: String, to: String, subject: String, body: String) -> Result<()> {
        match self {
            EmailProvider::MailDev(config) => {
                let email = Message::builder()
                    .from(from.parse().unwrap())
                    .to(to.parse().unwrap())
                    .subject(subject)
                    .header(ContentType::TEXT_HTML)
                    .body(body)
                    .map_err(|err| {
                        tracing::error!(?err, "Failed to build email message");
                        MailErrorCode::InvalidMailPayload(err)
                    })?;

                let mailer = SmtpTransport::from_url(config.smtp_host.as_ref())
                    .map_err(|err| {
                        tracing::error!(?err, "Failed to build email message");
                        MailErrorCode::SmtpConnection(err)
                    })?
                    .build();

                mailer.send(&email).map_err(|err| {
                    tracing::error!(?err, "Failed to build email message");
                    MailErrorCode::SmtpConnection(err)
                })?;

                Ok(())
            }
        }
    }
}

pub struct MailService {
    pub provider: EmailProvider,
}

impl MailService {
    pub fn new(config: &CommuneConfig) -> Self {
        let provider = EmailProvider::new(config);

        Self { provider }
    }

    pub async fn send_mail(
        &self,
        from: String,
        to: String,
        subject: String,
        body: String,
    ) -> Result<()> {
        self.provider.send_mail(from, to, subject, body)
    }
}
