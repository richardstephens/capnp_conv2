# Changelog

## 0.5

### 0.5.0-dev

- Add a `capnp_conv2_codegen_req` crate for working with`CodeGeneratorRequest` messages.
- Add support self-referential types, by supporting `Box<T>` and `Arc<T>` fields and
  enum variants.
- Fix `#[capnp_conv(name = "...")]` so that it works with capnp fields with a name that
  conflicts with a rust reserved word
- Bump capnp and capnpc to 0.27, which required adding a `'static` bound on generated
  `Readable`, `Writable`, and `TryFrom` impls.

## 0.4

### 0.4.0

- Forked from [capnp_conv](https://github.com/aikalant/capnp_conv) by @aikalant,
  renamed crates to `capnp_conv2` and `capnp_conv2_macros`.
- Update capnp and capnpc to 0.25.

## 0.3

### 0.3.1

- Update to capnp to 0.19 (@rschulman)
- Update heck to 0.5
- gitignored Cargo.lock
- Made readme global
- Removed unecessary `reborrow()` calls in `readable` trait codegen.
