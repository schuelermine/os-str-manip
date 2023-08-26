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
type OsStrItemRaw = u8;

#[cfg(not(doc))]
#[cfg(target_family = "windows")]
pub type OsStrItemRaw = u16;

#[cfg(doc)]
type OsStrItemRaw = PlatformSpecificType;

#[derive(Clone, Copy)]
pub struct OsStrItem(OsStrItemRaw);

type OsStrVec = Vec<OsStrItemRaw>;

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

impl From<OsStrItemRaw> for OsStrItem {
    fn from(value: OsStrItemRaw) -> Self {
        OsStrItem(value)
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
    fn starts_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    fn ends_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    fn strip_prefix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString>;
    fn strip_suffix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString>;
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
    fn starts_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool {
        pat.is_prefix_of(self)
    }
    fn ends_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool {
        pat.is_suffix_of(self)
    }
    fn strip_prefix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString> {
        pat.strip_prefix_of(self)
    }
    fn strip_suffix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString> {
        pat.strip_suffix_of(self)
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
            .take(self.end() + 1)
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source
            .items()
            .skip(*self.start())
            .take(self.end().unchecked_add(1))
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
        source.items().take(self.end + 1).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> OsString {
        source
            .items()
            .take(self.end.unckeched_add(1))
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

pub trait OsStrItemsIter: Iterator<Item = OsStrItem> + Sized {
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn to_os_string(self) -> OsString {
        OsStr::from_bytes(&self.map_raw().collect::<OsStrVec>()).to_os_string()
    }
    #[cfg(target_family = "windows")]
    fn to_os_string(self) -> OsString {
        OsString::from_wide(&self.map_raw().collect::<OsStrVec>())
    }
    fn map_raw(self) -> Map<Self, fn(Self::Item) -> OsStrItemRaw> {
        self.map(OsStrItem::raw)
    }
}

impl<T: Iterator<Item = OsStrItem>> OsStrItemsIter for T {}

pub trait OsStrItemsRawIter: Iterator<Item = OsStrItemRaw> + Sized {
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn to_os_string(self) -> OsString {
        OsStr::from_bytes(&self.collect::<OsStrVec>()).to_os_string()
    }
    #[cfg(target_family = "windows")]
    fn to_os_string(self) -> OsString {
        OsString::from_wide(&self.collect::<OsStrVec>())
    }
}

#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::slice::Iter<'a, OsStrItemRaw>);

#[cfg(target_family = "windows")]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

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

pub trait OsStrPattern<'a>: Sized {
    type Searcher: OsStrSearcher;

    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher;

    fn is_contained_in(self, haystack: &'a OsStr) -> bool {
        self.into_searcher(haystack).next_match().is_some()
    }
    fn is_prefix_of(self, haystack: &'a OsStr) -> bool {
        matches!(
            self.into_searcher(haystack).next(),
            OsStrSearchStep::Match(0, _)
        )
    }
    fn is_suffix_of(self, haystack: &'a OsStr) -> bool {
        let mut searcher = self.into_searcher(haystack);
        loop {
            match searcher.next() {
                OsStrSearchStep::Done => return false,
                OsStrSearchStep::Match(_, end) if end == haystack.len() => return true,
                _ => continue,
            }
        }
    }
    fn strip_prefix_of(self, haystack: &'a OsStr) -> Option<OsString> {
        if let OsStrSearchStep::Match(start, len) = self.into_searcher(haystack).next() {
            debug_assert_eq!(start, 0, "OsStrSearcher::next().0 must be 0 on first call");
            Some(haystack.index(len..))
        } else {
            None
        }
    }
    fn strip_suffix_of(self, haystack: &'a OsStr) -> Option<OsString> {
        let mut searcher = self.into_searcher(haystack);
        loop {
            match searcher.next() {
                OsStrSearchStep::Done => return None,
                OsStrSearchStep::Match(start, end) if end == haystack.len() => {
                    return Some(haystack.index(..start))
                }
                _ => continue,
            }
        }
    }
}

pub trait OsStrSearcher {
    fn next(&mut self) -> OsStrSearchStep;

    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                OsStrSearchStep::Match(a, b) => return Some((a, b)),
                OsStrSearchStep::Done => return None,
                _ => continue,
            }
        }
    }
    fn next_reject(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                OsStrSearchStep::Match(a, b) => return Some((a, b)),
                OsStrSearchStep::Done => return None,
                _ => continue,
            }
        }
    }
}

pub enum OsStrSearchStep {
    Match(usize, usize),
    Reject(usize, usize),
    Done,
}

impl<'a> OsStrPattern<'a> for OsStrItem {
    type Searcher = OsStrItemSearcher<'a>;

    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher {
        OsStrItemSearcher {
            haystack: haystack.items(),
            finger: 0,
            needle: self.raw(),
        }
    }
}

pub struct OsStrItemSearcher<'a> {
    haystack: OsStrItems<'a>,
    finger: usize,
    needle: OsStrItemRaw,
}

impl OsStrSearcher for OsStrItemSearcher<'_> {
    fn next(&mut self) -> OsStrSearchStep {
        let result = match self.haystack.next() {
            Some(item) if item.raw() == self.needle => {
                OsStrSearchStep::Match(self.finger, self.finger + 1)
            }
            Some(_) => OsStrSearchStep::Reject(self.finger, self.finger + 1),
            None => OsStrSearchStep::Done,
        };
        self.finger += 1;
        result
    }
}
