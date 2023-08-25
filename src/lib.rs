#![cfg_attr(
    feature = "generic_map_raw",
    feature(return_position_impl_trait_in_trait)
)]
#![cfg_attr(feature = "unchecked_index", feature(unchecked_math))]

#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
mod os_str_manip;

#[cfg(any(target_os = "wasi", target_family = "unix", target_family = "windows"))]
pub use os_str_manip::*;
