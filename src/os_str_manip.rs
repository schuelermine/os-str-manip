#![allow(clippy::wrong_self_convention)]

use std::ffi::{OsStr, OsString};
use std::iter::Map;

#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

#[cfg(target_family = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};

#[cfg(doc)]
#[doc(hidden)]
struct PlatformSpecificType;

#[cfg(doc)]
impl PlatformSpecificType {
    fn from_any<T>(_: T) {
        PlatformSpecificType
    }
}

#[cfg(not(doc))]
#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone, Copy)]
pub struct OsStrItem(u8);

#[cfg(not(doc))]
#[cfg(any(target_os = "wasi", target_family = "unix"))]
type OsStrItemRaw = u8;

#[cfg(not(doc))]
#[cfg(target_family = "windows")]
#[derive(Clone, Copy)]
pub struct OsStrItem(u16);

#[cfg(not(doc))]
#[cfg(target_family = "windows")]
type OsStrItemRaw = u16;

#[cfg(doc)]
#[derive(Clone, Copy)]
pub struct OsStrItem(());

#[cfg(doc)]
type OsStrItemRaw = PlaformSpecificType;

impl OsStrItem {
    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    pub fn raw(self) -> OsStrItemRaw {
        self.0
    }
    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    pub fn raw(self) -> OsStrItemRaw {
        self.0
    }
    pub fn to_os_string(self) -> OsString {
        std::iter::once(self).to_os_string()
    }
}

#[cfg(doc)]
impl OsStrItem {
    pub fn raw(self) -> PlatformSpecificType {
        PlatformSpecificType
    }
}

pub trait OsStrManip {
    fn items(&self) -> OsStrItems<'_>;
    fn index(&self, idx: impl OsStrIndex) -> OsString;
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_unchecked(&self, idx: impl OsStrIndex) -> OsString;
}

impl OsStrManip for OsStr {
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn items(&self) -> OsStrItems<'_> {
        OsStrItems(self.as_bytes().iter())
    }
    #[cfg(target_family = "windows")]
    fn items(&self) -> OsStrItems<'_> {
        OsStrItems(self.encode_wide())
    }
    fn index(&self, idx: impl OsStrIndex) -> OsString {
        idx.index_of(self)
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_unchecked(&self, idx: impl OsStrIndex) -> OsString {
        idx.index_of_unchecked(self)
    }
}

pub trait OsStrIndex {
    fn index_of(self, source: &OsStr) -> OsString;
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString;
}

impl OsStrIndex for std::ops::Range<usize> {
    fn index_of(self, source: &OsStr) -> OsString {
        assert!(self.start <= self.end);
        source
            .items()
            .skip(self.start)
            .take(self.end - self.start)
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source
            .items()
            .skip(self.start)
            .take(self.end.unchecked_sub(self.start))
            .to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeFrom<usize> {
    fn index_of(self, source: &OsStr) -> OsString {
        source.items().skip(self.start).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source.items().skip(self.start).to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeFull {
    fn index_of(self, source: &OsStr) -> OsString {
        source.to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source.to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeInclusive<usize> {
    fn index_of(self, source: &OsStr) -> OsString {
        source
            .items()
            .skip(*self.start())
            .enumerate()
            .take_while(|(i, _)| i <= self.end())
            .map(|(_, item)| item)
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source
            .items()
            .skip(*self.start())
            .enumerate()
            .take_while(|(i, _)| i <= self.end())
            .map(|(_, item)| item)
            .to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeTo<usize> {
    fn index_of(self, source: &OsStr) -> OsString {
        source.items().take(self.end).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source.items().take(self.end).to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeToInclusive<usize> {
    fn index_of(self, source: &OsStr) -> OsString {
        source
            .items()
            .enumerate()
            .take_while(|(i, _)| *i <= self.end)
            .map(|(_, item)| item)
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source
            .items()
            .enumerate()
            .take_while(|(i, _)| *i <= self.end)
            .map(|(_, item)| item)
            .to_os_string()
    }
}

impl OsStrIndex for usize {
    fn index_of(self, source: &OsStr) -> OsString {
        source.items().nth(self).unwrap().to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source.items().nth(self).unwrap_unchecked().to_os_string()
    }
}

pub trait OsStrItemsIter: Sized {
    fn to_os_string(self) -> OsString;
    #[cfg(feature = "generic_map_raw")]
    fn map_raw(self) -> Map<Self, impl FnMut(OsStrItem) -> OsStrItemRaw>;
}

impl<T: Iterator<Item = OsStrItem>> OsStrItemsIter for T {
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn to_os_string(self) -> OsString {
        OsStr::from_bytes(&self.map(OsStrItem::raw).collect::<Vec<OsStrItemRaw>>()).to_os_string()
    }
    #[cfg(target_family = "windows")]
    fn to_os_string(self) -> OsString {
        OsString::from_wide(&self.map(OsStrItem::raw).collect::<Vec<OsStrItemRaw>>())
    }
    #[cfg(feature = "generic_map_raw")]
    fn map_raw(self) -> Map<Self, impl FnMut(OsStrItem) -> OsStrItemRaw> {
        self.map(OsStrItem::raw)
    }
}

#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::slice::Iter<'a, OsStrItemRaw>);

#[cfg(target_family = "windows")]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

impl<'a> OsStrItems<'a> {
    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    pub fn map_raw(self) -> Map<Self, impl FnMut(OsStrItem) -> OsStrItemRaw> {
        self.map(OsStrItem::raw)
    }
    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    pub fn map_raw(self) -> Map<Self, impl FnMut(OsStrItem) -> OsStrItemRaw> {
        self.map(OsStrItem::raw)
    }
    #[cfg(doc)]
    pub fn map_raw(self) -> Map<Self, impl FnMut(OsStrItem) -> PlatformSpecificType> {
        self.map(PlatformSpecificType::from_any)
    }
}

impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;
    
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(*self.0.next()?))
    }
    #[cfg(target_family = "windows")]
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(self.0.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub trait OsPattern<'a>: Sized {
    type Searcher: OsSearcher<'a>;

    fn into_searcher(self, haystack: &OsStr) -> Self::Searcher;

    fn is_contained_in(self, haystack: &OsStr) -> bool {
        self.into_searcher(haystack).next_match().is_some()
    }
    fn is_prefix_of(self, haystack: &OsStr) -> bool {
        matches!(
            self.into_searcher(haystack).next(),
            OsSearchStep::Match(0, _)
        )
    }
    fn is_suffix_of(self, haystack: &OsStr) -> bool {
        let mut searcher = self.into_searcher(haystack);
        loop {
            match searcher.next() {
                OsSearchStep::Done => return false,
                OsSearchStep::Match(_, end) if end == haystack.len() => return true,
                _ => continue,
            }
        }
    }
    fn strip_prefix_of(self, haystack: &OsStr) -> Option<OsString> {
        if let OsSearchStep::Match(start, len) = self.into_searcher(haystack).next() {
            debug_assert_eq!(start, 0, "OsSearcher::next().0 must be 0 on first call");
            Some(haystack.index(len..))
        } else { None }
    }
    fn strip_suffix_of(self, haystack: &OsStr) -> Option<OsString> {
        let mut searcher = self.into_searcher(haystack);
        loop {
            match searcher.next() {
                OsSearchStep::Done => return None,
                OsSearchStep::Match(start, end) if end == haystack.len() => return Some(haystack.index(..start)),
                _ => continue,
            }
        }
    }
}

pub trait OsSearcher<'a> {
    fn haystack(&self) -> &'a [OsStrItem];
    fn next(&mut self) -> OsSearchStep;
    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                OsSearchStep::Match(a, b) => return Some((a, b)),
                OsSearchStep::Done => return None,
                _ => continue,
            }
        }
    }
    fn next_reject(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                OsSearchStep::Match(a, b) => return Some((a, b)),
                OsSearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub enum OsSearchStep {
    Match(usize, usize),
    Reject(usize, usize),
    Done,
}
