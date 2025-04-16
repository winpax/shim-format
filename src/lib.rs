#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    rustdoc::all,
    rust_2024_compatibility,
    missing_docs
)]

#[cfg(feature = "deserializing")]
mod deserializing;
#[cfg(feature = "serializing")]
mod serializing;

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        macro_rules! prelude {
            () => {};
        }
    } else {
        macro_rules! prelude {
            () => {
                #[allow(unused_imports)]
                use alloc::{string::{String, ToString}, vec::Vec};
            };
        }
    }
}

pub(crate) use prelude;

prelude!();

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when parsing or serializing a shim.
pub enum Error {
    #[cfg(feature = "deserializing")]
    #[error("{0}")]
    /// Deserializing errors
    ///
    /// See [`deserializing::Error`] for more information
    DeserializingError(#[from] deserializing::Error),

    #[cfg(feature = "std")]
    #[error("{0}")]
    /// Reading from a reader errors
    ///
    /// See [`std::io::Error`] for more information
    ReadingError(#[from] std::io::Error),

    #[error("{0}")]
    /// Writing to a writer errors
    ///
    /// See [`core::fmt::Error`] for more information
    WritingError(#[from] core::fmt::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Scoop shim struct
///
/// This holds all known supported data
/// that a Scoop shim file can provide
pub struct Shim {
    path: String,
    args: Vec<String>,
}

impl Shim {
    #[must_use]
    /// Construct a new [`Shim`]
    pub fn new(path: String, args: Vec<String>) -> Self {
        Self { path, args }
    }

    #[must_use]
    /// Get a reference to the shim's path
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    /// Get a reference to the shim's arguments
    pub fn args(&self) -> &[String] {
        &self.args
    }
}

#[inline]
#[cfg(feature = "deserializing")]
/// Parse a [`Shim`] from a string
///
/// This is a wrapper around [`Shim::from_str`].
///
/// # Errors
/// Parsing the shim. See [`Error`] for more details.
pub fn from_str(s: &str) -> Result<Shim, Error> {
    use core::str::FromStr;

    Ok(Shim::from_str(s)?)
}

#[cfg(all(feature = "std", feature = "deserializing"))]
#[inline]
/// Parse a [`Shim`] from a reader
///
/// Note that this collects the reader's data into a string
/// and then passes that to [`from_str`].
///
/// This is purely a convenience method, and provides no
/// benefits regarding memory usage or performance.
///
/// # Errors
/// Parsing the shim. See [`Error`] for more details.
///
/// Reading from the reader. See [`std::io::Error`] and [`std::io::Read::read_to_string`] for more details.s
pub fn from_reader(reader: &mut impl std::io::Read) -> Result<Shim, Error> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    from_str(&buf)
}

#[inline]
#[must_use]
#[cfg(feature = "serializing")]
/// Serialize a [`Shim`] to a string
///
/// This is a wrapper around [`Shim::to_string`]
pub fn to_string(shim: &Shim) -> String {
    alloc::string::ToString::to_string(shim)
}

#[inline]
#[cfg(feature = "serializing")]
/// Write the shim to a writer
///
/// # Errors
/// Writing to the writer. See [`core::fmt::Error`] for more details.
pub fn to_writer(shim: &Shim, writer: &mut impl core::fmt::Write) -> Result<(), Error> {
    Ok(writer.write_str(&shim.to_string())?)
}
