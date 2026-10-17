# capnp codegen request types

Rust structs for reading capnp codegen requests, intended to
be used for code generators that need to work with a parsed capnp schema.

This does not yet fully capture every upstream schema field, but should be
enough for most purposes.

Doc comments copied from upstream's
[schema.capnp](https://github.com/capnproto/capnproto/blob/master/c%2B%2B/src/capnp/schema.capnp).
