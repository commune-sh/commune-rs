use handlebars::Handlebars;
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

pub enum EmailTemplate {
    VerificationCode { name: String, code: String },
}

impl EmailTemplate {
    pub fn template(&self) -> &'static str {
        match self {
            EmailTemplate::VerificationCode { .. } => {
                include_str!("templates/verification_code.hbs")
            }
        }
    }

    pub fn subject(&self) -> String {
        match self {
            EmailTemplate::VerificationCode { .. } => "Verification Code".to_owned(),
        }
    }

    pub fn data(&self) -> serde_json::Value {
        match self {
            EmailTemplate::VerificationCode { name, code } => serde_json::json!({
                "name": name,
                "code": code,
            }),
        }
    }

    pub fn render(&self, hbs: &Handlebars<'static>) -> Result<String> {
        match self {
            EmailTemplate::VerificationCode { .. } => {
                let data = self.data();
                let template = self.template();
                let html = hbs.render_template(template, &data).map_err(|err| {
                    tracing::error!(?err, "Failed to render handlebars template");
                    MailErrorCode::RenderHandlebars(err)
                })?;

                Ok(html)
            }
        }
    }
}

pub struct MailService {
    pub hbs: Handlebars<'static>,
    pub provider: EmailProvider,
}

impl MailService {
    pub fn new(config: &CommuneConfig) -> Self {
        let provider = EmailProvider::new(config);
        let hbs = Handlebars::new();

        Self { hbs, provider }
    }

    pub async fn send_mail(
        &self,
        from: String,
        to: String,
        subject: String,
        template: EmailTemplate,
    ) -> Result<()> {
        let body = template.render(&self.hbs)?;

        self.provider.send_mail(from, to, subject, body)
    }
}
