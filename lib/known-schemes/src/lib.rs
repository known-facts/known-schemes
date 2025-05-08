// This is free and unencumbered software released into the public domain.

//! This crate provides well-known URI/IRI schemes.
//!
//! ```edition2024
//! # use known_schemes::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

mod features;
pub use features::*;

mod iri_scheme;
pub use iri_scheme::*;

mod prelude;

mod uri_scheme;
pub use uri_scheme::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
