//! Structured log types.
//!
//! ## Example
//! ```rust
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// Delimit a span.
pub enum Span {
    /// Marks the end of a span.
    End,
    /// Marks the start of a span.
    Start,
}

/// Delimit an HTTP request
pub enum Http {
    /// Marks an HTTP Request.
    Request,
    /// Marks an HTTP Response.
    Response,
}
