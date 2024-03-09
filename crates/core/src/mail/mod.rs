use tokio::net::TcpStream;

use mail_send::{
    mail_builder::{headers::address::EmailAddress, MessageBuilder},
    SmtpClient, SmtpClientBuilder,
};
use matrix::ruma_common::ServerName;
use rand::Rng;

use crate::error::Error;

pub struct Mail {
    conn: SmtpClient<TcpStream>,
}

impl Mail {
    pub async fn build(host: &ServerName) -> SmtpClient<TcpStream> {
        tracing::warn!("using Mailtutan as email gateway, this is only meant for development");

        for attempt in 1..=3 {
            match SmtpClientBuilder::new(host.as_str(), 587)
                .implicit_tls(false)
                .credentials(("admin", "admin"))
                .connect_plain()
                .await
            {
                Ok(conn) => return conn,
                Err(e) => {
                    if attempt >= 3 {
                        panic!("failed to connect to the SMTP host: {e}")
                    }
                }
            }
        }

        unreachable!()
    }

    pub async fn send_verification(
        &mut self,
        server_name: &ServerName,
        recipient: EmailAddress<'_>,
    ) -> Result<(), Error> {
        let message = MessageBuilder::new()
            .from(format!("onboarding@{}", server_name))
            .to(&*recipient.email)
            .subject("Please verify your email address")
            .html_body(
                template::Verification::build(
                    server_name,
                    &recipient.name.expect("recipient name cannot be empty"),
                    rand::thread_rng().gen(),
                )
                .into_string(),
            );

        self.conn.send(message).await.map_err(|e| {
            tracing::error!(?e, "failed to send message to SMTP host");

            Error::SMTP(e)
        })?;

        Ok(())
    }
}

mod template {
    use matrix::ruma_common::ServerName;
    use maud::{html, PreEscaped, DOCTYPE};

    pub struct Verification;

    impl Verification {
        pub fn build(
            server_name: &ServerName,
            username: &str,
            code: [u8; 6],
        ) -> PreEscaped<String> {
            html! {
                (DOCTYPE)
                html lang="en" {
                    header {
                        (PreEscaped(STYLESHEET))
                        title { "Welcome to Commune" }

                        meta charset="utf-8";
                        meta name="viewport" content="width=device-width, initial-scale=1";
                    }
                    body {
                        div ."halftone-div";
                        section {
                            div .title {
                                h3 .header {
                                    "Commune - " (username) ", welcome abroad"
                                }
                                h5 .greeting {
                                    "We are glad that you have decided to join the network. \
                                    To finish setting up your account, enter this code in your client."
                                }
                            }
                            code {
                                span {
                                    (code.iter().map(|n| char::from_digit(*n as u32, 10).expect("code should only contain digits")).collect::<String>())
                                }
                            }
                            footer {
                                p .disclaimer {
                                    "If you didn't request a code, you can safely ignore this email."
                                }
                                p .contact {
                                    "This email was sent to you by a"
                                    (PreEscaped(
                                        "<a href=\"https://commune.sh\" \
                                        style=\"text-decoration-skip-ink: auto; text-decoration: underline solid 1px; text-underline-offset: 1px; color: #555;\"> \
                                        Commune instance </a>"
                                    ))
                                    "If you have any questions, please get in touch with the administrator of " (server_name) "."
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    const STYLESHEET: &str = r#"
      <style>
        * {
          text-transform: capitalize;
          padding: 0;
          margin: 0;
          box-sizing: border-box;
          font-family: "Montserrat", "Segoe UI", Roboto, sans-serif;
        }

        body {
          display: flex;
          justify-content: center;
          align-items: center;

          width: 100%;
          height: 100vh;

          background: rgb(199, 255, 232);
          background: linear-gradient(90deg, rgba(199, 255, 232, 1) 0%, rgba(255, 255, 255, 1) 30%, rgba(97, 149, 255, 1) 100%);
        }

        section {
          display: flex;
          flex-direction: column;
          gap: 4rem;
          width: 60em;
          min-height: 28em;
          padding: 1.5rem 3rem;

          background-color: white;
          background-color: rgba(255, 255, 255, .1);
          border: 4px solid rgba(255, 255, 255, .5);
          border-radius: 8px;
          box-shadow: 0 0 10px 1px rgba(0, 0, 0, 0.25);
          backdrop-filter: blur(2px);
          z-index: 1;

        }

        section .title {
          display: flex;
          gap: 1rem;
          flex-direction: column;
          align-items: center;
        }

        section .title .header {
          width: 100%;
          color: #111;
          font-size: xx-large;
          line-height: 28px;
          font-weight: 800;
          text-align: center;
        }

        section .title .greeting {
          width: 75%;
          color: #444;
          font-size: 13px;
          line-height: 20px;
          font-weight: 600;
          text-align: center;
        }

        section code {
          max-width: fit-content;
          margin-left: auto;
          margin-right: auto;
          padding: 0 1rem;

          background: radial-gradient(circle at center, transparent 0rem, #0004 24rem);
          border: 6px double #333;
          border-radius: 4px;
          font-size: 48px;
          font-weight: 600;
          letter-spacing: 0.5rem;
          color: #FFF;
        }

        section code span {
          filter: drop-shadow(2px 2px 0.5px #0008);
        }

        section .code-section .code-box2 {
          display: flex;
          align-items: center;
          justify-content: center;
          width: 91px;
          height: 125px;
          border: 2px solid #627ad1;
          border-radius: 10px;
        }

        section footer {
          font-size: medium;
          font-weight: 600;
          text-align: center;
          color: #111;

          display: flex;
          flex-direction: column;
          justify-content: center;
          gap: 2rem;
        }

        section footer .contact {
          padding: 1rem;
          background: white;
          border: 2px solid #222;

          position: relative;
          font-size: x-small;
          font-weight: 600;
          color: #444;
          box-shadow: 0px 0px 4px #2228;
        }

        section footer .contact::before {
          display: block;
          background: #222;
          width: 100%;
          height: 100%;

          content: "";
          position: absolute;
          bottom: 4px;
          right: 4px;
          border: 2px solid #222;
          background-image: repeating-linear-gradient(-45deg, transparent, transparent 2px, white 0, white 3px);
          image-rendering: auto;
          z-index: -1;
        }

        .halftone {
          width: 100%;
          height: 100vh;
          position: absolute;
          background: black;
          filter: contrast(50);
          mix-blend-mode: screen;
        }

        .halftone::after {
          content: '';
          position: absolute;
          inset: 0;

          background-image: radial-gradient(circle at center, #FFF 0.06rem, transparent 0.66rem),
            radial-gradient(circle at center, #FFF 0.06rem, transparent 0.66rem);
          mask-image: linear-gradient(-30deg, #000, rgba(0, 0, 0, 0.25));
          background-position: 0 0, 0.5rem 0.5rem;
          background-size: 1rem 1rem;
          background-repeat: round;
        }
      </style>
    "#;
}
