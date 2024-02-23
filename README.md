<div align="center">
  <h1 align="center">commune-rs</h1>
  <h3 align="center">
    Commune Server written in Rust
  </h3>
</div>

Commune allows you to create free, open and public communities on top
of the [Matrix](https://matrix.org) protocol. It allows homeservers to expose
(a subset of) spaces and rooms to the world wide web, extending them with
extra community features, such as discussion boards and threaded comments.

A comprehensive introduction of the concept is offered in the article about [Communal Bonfires](https://blog.erlend.sh/communal-bonfires).

We currenly aim to reimplement [commune-server](commune-server),
which is written in Golang. The goal is to prove that Rust is fit
for backend programming and offers a variety of exclusive benefits over other languagestab=readme-ov-file
that encourage good practices while retaining low-level control over details.


#### Live instances

- [shpong.com](https://shpong.com) - Reddit-like
- [commune.sh](https://commune.sh) - Gitter-like

#### Installation

We currently only have an alpha version available that runs as a collection of Docker containers.

- If you didn't already, install the Rust toolchain through your package manager or [rustup](https://rustup.rs).
- Install Just, releases can be found [here](https://github.com/casey/just#packages).

- Create a configuration file for Synapse with `just gen_synapse_conf`, it should be located at `docker/synapse`.
- Change the variables found in `.env` to match your environment.
- Start running the Docker environment with `just backend`.
- Register an admin account and retrieve the access token with `just gen_synapse_admin && just get_access_token`.
- Compile and run the binary with `cargo r --release`.

#### Short-term roadmap
- [ ] Porting over the base functionality of [commune-server](https://github.com/commune-os/commune-server)
- [ ] Federation between Commune instances
- [ ] SSO login support through OpenID Connect
- [ ] ActivityPub support for interacting with the fediverse
- [ ] Private spaces/boards and Encrypted DMs
- [ ] Simplify self-hosting deployment

#### Development

Prepare the development environment and run the project locally by following
the [contributor guide](CONTRIBUTING.md).

#### License

This project is licensed under the Apache License Version 2.0
