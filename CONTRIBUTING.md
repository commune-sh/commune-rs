There are many ways to contribute to Commune. We're currently still in the alpha phase of porting [commune-server](https://github.com/commune-os/commune-server).
Although it's proven to be a useful reference, breaking changes are expected as a frontend rewrite is on its way as well.


Opening pull requests, suggesting new features, writing tests, etc. are more than welcomed and deeply appreciated.
If you need guidance feel free to join our Matrix room [#commune-dev:kurosaki.cx](https://matrix.to/#/#commune-dev:kurosaki.cx) or jump by on our [Discord](https://discord.gg/W9mbH5F36J). We do not recommend the usage of Discord since it's proprietary software with questionable practices when it comes to privacy but we have to yet plan a bridge setup.


### Getting Started

The setup instructions are the same as for a regular installation, except that we prepared
a special [homeserver.yaml](crates/test/fixtures/synapse/homeserver.yaml) which is used for testing.
Copy this over to `docker/synapse/homeserver.yaml` and you're ready to go!


### Testing

Unit tests Are inlined inside crates and should be used to test the behavior of types such
as (de)serialization, validation and idempotency. These are simply ran by invoking `cargo test -p <crate name>`.


Integration (e2e) tests are defined in the `test` crate and should confirm that all components
communicate with each other the right way. As this is more code-intense, there's been an effort
to define several helper functions in [util.rs](crates/test/src/util.rs) to reduce repetiveness.


### Application Architecture

### Styling Conventions

### Submitting PRs
