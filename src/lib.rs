#![cfg_attr(feature = "unchecked_index", feature(unchecked_math))]

//! Tools for manipulating [`OsStr`]s and [`OsString`]s
//!
//! It supports WASI, Unix, and Windows, as these are the only platforms that provide
//! basic [`OsStr`] inspection via an `OsStrExt` trait
//!
//! The crate is not optimized for each OS—instead, it defines core functionality for
//! interfacing with [`OsStr`]s and uses them to provide its functionality with
//! cross-platform algorithms
//!
//! The main entry point is the [`OsStrManip`] trait, and the [`prelude`] module
//!
//! This crate has an optional feature that requires unstable features
//! and therefore nightly rust: `unchecked_index`, which provides [`OsStrManip::index_unchecked`]
//! and [`OsStrIndex::index_of_unchecked`]
//!
//! [`OsStr`]: std::ffi::OsStr
//! [`OsString`]: std::ffi::OsString
//! [`OsStrManip`]: prelude::OsStrManip
//! [`OsStrManip::index_unchecked`]: prelude::OsStrManip::index_unchecked
//! [`OsStrIndex::index_of_unchecked`]: prelude::OsStrIndex::index_of_unchecked

/// This module contains all the functionality of this crate intended for public use
#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
pub mod prelude {
    pub use super::os_str_manip::{
        OsStrIndex, OsStrManip, OsStrPattern, OsStringFromItem, OsStringFromIter,
    };
}

/// This module contains the entire public API including items that aren’t
/// intended for public use
#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
pub mod os_str_manip;
