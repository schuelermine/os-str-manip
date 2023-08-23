#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStrExt;

use std::ffi::{OsStr, OsString};

#[cfg(target_family = "windows")]
pub struct OsStrItem(u16);

#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone, Copy)]
pub struct OsStrItem(u8);

pub trait OsStrManip {
    fn iter_items<'a>(&'a self) -> OsStrItems<'a>;
}

#[cfg(any(target_os = "wasi", target_family = "unix"))]
impl OsStrManip for OsStr {
    fn iter_items<'a>(&'a self) -> OsStrItems<'a> {
        OsStrItems(self.as_bytes().iter())
    }
}

#[cfg(target_family = "windows")]
impl OsStrManip for OsStr {
    fn iter_items<'a>(&'a self) -> OsStrItems<'a> {
        OsStrItems(self.encode_wide())
    }
}

#[cfg(target_family = "windows")]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

#[cfg(any(target_os = "wasi", target_family = "unix"))]
pub struct OsStrItems<'a>(std::slice::Iter<'a, u8>);

#[cfg(target_family = "windows")]
impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(self.0.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[cfg(any(target_os = "wasi", target_family = "unix"))]
impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(*self.0.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub trait OsPattern<'a>: Sized {
    type Searcher: OsSearcher<'a>;

    fn into_searcher(self, haystack: &OsStr) -> Self::Searcher;

    fn is_contained_in(self, haystack: &OsStr) -> bool {
        todo!()
    }
    fn is_prefix_of(self, haystack: &OsStr) -> bool {
        todo!()
    }
    fn is_suffix_of(self, haystack: &OsStr) -> bool
    where
        Self::Searcher: OsReverseSearcher<'a>,
    {
        todo!()
    }
    fn strip_prefix_of(self, haystack: &OsStr) -> OsString {
        todo!()
    }
    fn strip_suffix_of(self, haystack: &OsStr) -> OsString
    where
        Self::Searcher: OsReverseSearcher<'a>,
    {
        todo!()
    }
}

pub trait OsSearcher<'a> {
    fn haystack(&self) -> &'a [OsStrItem];
    fn next(&mut self) -> OsSearchStep;

    fn next_match(&mut self) -> Option<(usize, usize)> {
        todo!()
    }
    fn next_reject(&mut self) -> Option<(usize, usize)> {
        todo!()
    }
}

pub trait OsReverseSearcher<'a>: OsSearcher<'a> {
    fn next_back(&mut self) -> OsSearchStep;

    fn next_match_back(&mut self) -> Option<(usize, usize)> {
        todo!()
    }
    fn next_reject_back(&mut self) -> Option<(usize, usize)> {
        todo!()
    }
}

pub enum OsSearchStep {
    Match(usize, usize),
    Reject(usize, usize),
    Done,
}
