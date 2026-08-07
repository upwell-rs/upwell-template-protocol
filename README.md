# {{ project-name }}

A first-class Upwell protocol generated with `cargo upwell init` or `cargo generate`.

The template demonstrates the complete protocol lifecycle:

- definition and validation;
- a protocol-owned scope topology;
- prepared state and runtime construction;
- serving over a concrete endpoint type;
- optional tooling metadata.

Replace the `()` endpoint with the transport accepted by the real protocol.

Set `macro_crate = true` when generating to add a companion proc-macro crate and re-exported
`#[protocol_component]` extension point built on `upwell-macros-core`.

## License

Generated projects are configured for `MIT OR Apache-2.0`. Replace that package metadata if your
project uses another license. The template repository itself is available under MIT.
