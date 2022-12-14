#![deny(clippy::pedantic)]
//! `packfile` is a simple library providing utilities to generate [Git Packfiles] in memory.
//!
//! Usage:
//!
//! ```rust
//! # use packfile::{high_level::GitRepository, low_level::PackFile};
//! #
//! let mut repo = GitRepository::default();
//! repo.insert(&["path", "to"], "file.txt", "hello world!".into()).unwrap();
//! let (_commit_hash, entries) =
//!     repo.commit("Linus Torvalds", "torvalds@example.com", "Some commit message").unwrap();
//!
//! let _packfile = PackFile::new(&entries);
//! ```
//!
//! The generated packfile can then be encoded within a [`SidebandData`] to send the data to a
//! client
//!
//! [Git Packfiles]: https://git-scm.com/book/en/v2/Git-Internals-Packfiles
//! [`SidebandData`]: crate::codec::Codec::SidebandData

#[cfg(feature = "tokio-util")]
pub mod codec;
mod error;
pub mod high_level;
pub mod low_level;
mod packet_line;
mod util;

pub use error::Error;
pub use packet_line::PktLine;
