<div align="center">
  <h1 align="center">commune</h1>
  <p align="center">
    Commune Server
  </p>
</div>

## Development

### Requirements

- [Docker](https://www.docker.com/get-started/)
- [Justfile](https://github.com/casey/just)
- [Rust](https://rustup.rs)

### Getting Started

1. Create a copy of `.env.example` on `.env`

```bash
cp .env.example .env
```

2. Generate `Synapse` server configuration

```bash
just gen_synapse_conf
```

3. Run Synapse Server (and other containerized services) using Docker Compose
via:

```bash
just backend
```

**When you are ready**

Teardown services using `just stop`. If you want to perform a complete cleanup
use `just clear`.

> **Warning** `just clear` will remove all containers and images.

### Testing

This application has 2 layers for tests:

- `Unit`: Are usually inlined inside crates, and dont depend on any integration
- `E2E`: Lives in `test` crate and counts with the services that run the application

#### Unit

Unit tests can be executed via `cargo test -p <crate name>`, this will run
every unit test.

#### E2E

You must run Docker services as for development. In order to avoid messing up
the development environment, its recommended to use the synapse setup from
`crates/test/fixtures/synapse` replacing it with `docker/synapse`.

The only difference should be the `database` section, which uses SQLite instead.

```diff
database:
+  name: psycopg2
+  args:
+    database: /data/homeserver.db
-  name: psycopg2
-  txn_limit: 10000
-  allow_unsafe_locale: true
-  args:
-    user: synapse_user
-    password: secretpassword
-    database: synapse
-    host: synapse_database
-    port: 5432
-    cp_min: 5
-    cp_max: 10
```

> Make sure the `.env` file is created from the contents on `.env.example`

### Application Layout

<div align="center">
  <img src="./docs/diagrams/diagram.png" />
  <small>Application Layout Overview</small>
</div>

The client, any HTTP Client, comunicates with the Commune Server which may or
may not communicate with Matrix's server _Synapse_ which runs along with its
database in a Docker container.

## License

This project is licensed under the Apache License Version 2.0
