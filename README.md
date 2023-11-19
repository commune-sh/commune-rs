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

1. Generate `Synapse` server configuration

```bash
just gen_synapse_conf
```

2. Run Synapse Server (and other containerized services) using Docker Compose
via:

```bash
just backend
```

**When you are ready**

Teardown services using `just stop`. If you want to perform a complete cleanup
use `just clear`.

> **Warning** `just clear` will remove all containers and images.

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
