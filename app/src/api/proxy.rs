use std::net::SocketAddr;

use http::uri::Authority;
use http::StatusCode;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};

use tokio::net::{TcpListener, TcpStream};

use crate::Error;

pub(crate) struct Proxy {}

impl Proxy {
    pub(crate) async fn connect(addr: SocketAddr) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await.map_err(|_| Error::Todo(()))?;

        loop {
            let (stream, _) = listener.accept().await.map_err(|_| Error::Todo(()))?;

            let http = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(hyper_util::rt::TokioIo::new(stream), service_fn(proxy))
                .with_upgrades()
                .await;

            tokio::task::spawn(async move {
                if let Err(err) = http {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }
}

async fn proxy(
    request: Request<Incoming>,
) -> Result<Response<BoxBody<bytes::Bytes, hyper::Error>>, hyper::Error> {
    if let Method::CONNECT = *request.method() {
        let Some(addr) = request.uri().authority().map(Authority::to_string) else {
            tracing::error!(uri = ?request.uri(), "CONNECT host is not socket addr");

            let body = Full::new("CONNECT must be to a socket address".into())
                .map_err(|never| match never {})
                .boxed();

            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .expect("StatusCode is valid"));
        };

        tokio::spawn(async move {
            match hyper::upgrade::on(request)
                .await
                .map(hyper_util::rt::TokioIo::new)
            {
                Ok(mut upgraded) => {
                    let Ok(mut server) = TcpStream::connect(addr).await.inspect_err(|error| {
                        tracing::error!("server IO error: {error}");
                    }) else {
                        return;
                    };

                    match tokio::io::copy_bidirectional(&mut upgraded, &mut server).await {
                        Ok((from_client, from_server)) => {
                            tracing::info!(
                                "client wrote {from_client} bytes and received {from_server} bytes",
                            );
                        }
                        Err(error) => {
                            tracing::error!("server IO error: {error}");
                        }
                    };
                }
                Err(error) => {
                    eprintln!("upgrade error: {error}")
                }
            }
        });

        return Ok(Response::new(
            Empty::<bytes::Bytes>::new()
                .map_err(|never| match never {})
                .boxed(),
        ));
    }

    let stream = TcpStream::connect((
        request.uri().host().expect("uri has no host"),
        request.uri().port_u16().unwrap_or(80),
    ))
    .await
    .unwrap();

    let (mut sender, conn) = Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .handshake(hyper_util::rt::TokioIo::new(stream))
        .await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let response = sender.send_request(request).await?;

    Ok(response.map(Incoming::boxed))
}
