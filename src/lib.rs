#![cfg_attr(feature = "unchecked_index", feature(unchecked_math))]

//! Tools for manipulating [`std::ffi::OsStr`]s and [`std::ffi::OsString`]s.
//! It supports WASI, Unix, and Windows, as these are the only platforms that provide
//! basic [`std::ffi::OsStr`] inspection via an `OsStrExt` trait.
//! The library is not optimized for each OS. Instead, it defines core functionality for
//! interfacing with [`std::ffi::OsStr`]s and uses them to provide its functionality with
//! cross-platform algorithms.
//!
//! The main entry point is the [`prelude::OsStrManip`] trait.

#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
pub mod os_str_manip;

#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
pub mod prelude {
    pub use super::os_str_manip::{OsStrManip, OsStringFromItem, OsStringFromIter};
}
