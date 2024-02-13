use handlebars::Handlebars;
use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};
use url::Url;

use crate::{mail::error::MailErrorCode, util::secret::Secret, CommuneConfig, Result};

pub struct MailDevConfig {
    pub smtp_host: Url,
}

pub enum EmailProvider {
    MailDev(MailDevConfig),
}

impl EmailProvider {
    pub fn new(config: &CommuneConfig) -> Self {
        // TODO: Provide support for different providers via Config
        tracing::warn!(
            %config.smtp_host,
            "Using MailDev as email provider! This is only for development!"
        );

        EmailProvider::MailDev(MailDevConfig {
            smtp_host: config.smtp_host.to_owned(),
        })
    }

    pub fn send_mail(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<()> {
        match self {
            EmailProvider::MailDev(config) => {
                let email = Message::builder()
                    .from(from.parse().unwrap())
                    .to(to.parse().unwrap())
                    .subject(subject)
                    .header(ContentType::TEXT_HTML)
                    .body(body.to_owned())
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
    VerificationCode { code: Secret },
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
            EmailTemplate::VerificationCode { code } => serde_json::json!({
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

    pub async fn send_mail<S: AsRef<str>>(
        &self,
        from: S,
        to: S,
        template: EmailTemplate,
    ) -> Result<()> {
        let subject = template.subject();
        let body = template.render(&self.hbs)?;

        self.provider
            .send_mail(from.as_ref(), to.as_ref(), subject.as_str(), body.as_str())
    }
}
