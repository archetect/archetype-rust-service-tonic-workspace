# Archetype: Rust Modular gRPC Microservice

A Rust gRPC-based Microservice archetype, with the following qualities:

- Follows strong [Twelve Factor](https://12factor.net/) principles
  - Layered, hierarchical configuration using [config](https://github.com/mehcode/config-rs) 
  - Run and managed by an ergonomic CLI interface, powered by [clap](https://github.com/clap-rs/clap)
- Completely asynchronous, powered by [Tokio](https://tokio.rs/)
- Modular, with individually usable and tested layers:
  - Server: [gRPC](https://grpc.io/) based remoting layer using [Tonic](https://github.com/hyperium/tonic)
  - Core: business layer, adapting the gRPC API implementation over the persistence layer
  - Persistence: persistence tier abstraction and database migrations, provided by [SeaORM](https://github.com/SeaQL/sea-orm)

To generate a project from this archetype using [Archetect](https://github.com/archetect/archetect):

```shell
archetect render https://github.com/archetect/archetype-rust-service-tonic-workspace.git
```
