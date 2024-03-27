use std::net::SocketAddr;

pub(crate) struct Env {
    pub client: reqwest::Client,
    pub loopback: SocketAddr,
}

impl Env {
    pub(crate) async fn new() -> Self {
        let _ = tracing_subscriber::fmt().try_init();

        commune::init().await;

        let loopback = SocketAddr::from((
            match commune::commune().config.public_loopback {
                true => [0, 0, 0, 0],
                false => [127, 0, 0, 1],
            },
            5357,
        ));

        tokio::spawn(async move {
            tracing::info!("starting development server on {:?}", loopback);

            router::serve(commune::commune().config.public_loopback, 5357)
                .await
                .expect("failed to bind to address");
        });

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        if let Err(e) = client
            .get(commune::commune().config.matrix.host.to_string() + "/_matrix/client/versions")
            .send()
            .await
        {
            tracing::error!(
                "could not connect to Matrix: {e}\n is the testing environment running?"
            );

            std::process::exit(1);
        }

        Self { client, loopback }
    }

    fn path(&self, path: &str) -> String {
        format!("http://{}{}", self.loopback, path)
    }

    pub(crate) fn get(&self, url: &str) -> reqwest::RequestBuilder {
        tracing::info!("GET {}", self.path(url));

        self.client.get(self.path(url))
    }

    pub(crate) fn post(&self, url: &str) -> reqwest::RequestBuilder {
        tracing::info!("POST {}", self.path(url));

        self.client.post(self.path(url))
    }

    pub(crate) fn put(&self, url: &str) -> reqwest::RequestBuilder {
        tracing::info!("PUT {}", self.path(url));

        self.client.put(self.path(url))
    }

}
