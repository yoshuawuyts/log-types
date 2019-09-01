//! Structured log types.
//!
//! This crate is useful when creating shared log abstractions between crates; consistently marking
//! pairs of logs.
//!
//! See also: [async-log](https://docs.rs/async-log/),
//! [log::kv](https://docs.rs/log/0.4.8/log/kv/index.html),

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

mod http;
mod span;

pub use http::Http;
pub use span::Span;
