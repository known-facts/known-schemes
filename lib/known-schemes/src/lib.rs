// This is free and unencumbered software released into the public domain.

//! This crate provides an enum of well-known URI/IRI schemes.
//!
//! ```edition2024
//! # use known_schemes::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

mod features;
pub use features::*;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
