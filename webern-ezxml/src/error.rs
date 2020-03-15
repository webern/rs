// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Contains the error type for this library.

#![allow(clippy::default_trait_access)]

use std::path::PathBuf;

use snafu::{Backtrace, Snafu};

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ParseLocation {
    pub line: u64,
    pub column: u64,
}

/// The error type for this library.
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    /// A failure while parsing xml.
    #[snafu(display("Failure while parsing: {:?}", parse_location))]
    Parse {
        parse_location: ParseLocation,
        backtrace: Backtrace,
    },
    IoRead {
        parse_location: ParseLocation,
        source: std::io::Error,
        backtrace: Backtrace,
    },
}

// used in `std::io::Read` implementations
impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        Self::new(std::io::ErrorKind::Other, err)
    }
}
