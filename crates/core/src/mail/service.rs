use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::{CommuneConfig, Result};

pub struct MailDevConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub outgoing_user: String,
    pub outgoing_pass: String,
}

pub enum EmailProvider {
    MailDev(MailDevConfig),
}

impl EmailProvider {
    pub fn new(config: &CommuneConfig) -> Self {
        if let (
            Some(smtp_host),
            Some(smtp_port),
            Some(maildev_incoming_user),
            Some(maildev_incoming_pass),
        ) = (
            &config.smtp_host,
            &config.smtp_port,
            &config.maildev_incoming_user,
            &config.maildev_incoming_pass,
        ) {
            return EmailProvider::MailDev(MailDevConfig {
                smtp_host: smtp_host.clone(),
                smtp_port: *smtp_port,
                outgoing_user: maildev_incoming_user.clone(),
                outgoing_pass: maildev_incoming_pass.clone(),
            });
        }

        panic!("No email provider configured");
    }

    pub fn send_mail(&self, from: String, to: String, subject: String, body: String) {
        match self {
            EmailProvider::MailDev(config) => {
                let email = Message::builder()
                    .from(from.parse().unwrap())
                    .to(to.parse().unwrap())
                    .subject(subject)
                    .header(ContentType::TEXT_HTML)
                    .body(body)
                    .unwrap();

                let creds =
                    Credentials::new(config.outgoing_user.clone(), config.outgoing_pass.clone());

                let mailer = SmtpTransport::relay(&config.smtp_host)
                    .unwrap()
                    .port(config.smtp_port)
                    .credentials(creds)
                    .build();

                mailer.send(&email).unwrap();
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
        self.provider.send_mail(from, to, subject, body);
        Ok(())
    }
}
