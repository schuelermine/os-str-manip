#![allow(clippy::wrong_self_convention)]

use std::ffi::{OsStr, OsString};

#[cfg(not(doc))]
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(not(doc))]
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;

#[cfg(not(doc))]
#[cfg(target_family = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};

#[cfg(doc)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlatformSpecificType;

#[cfg(not(doc))]
#[cfg(any(target_os = "wasi", target_family = "unix"))]
type OsStrItem = u8;

#[cfg(not(doc))]
#[cfg(target_family = "windows")]
type OsStrItem = u16;

#[cfg(doc)]
type OsStrItem = PlatformSpecificType;

#[cfg(not(doc))]
type OsStrVec = Vec<OsStrItem>;

mod os_string_from_item_sealed {
    use super::OsStrItem;

    pub trait Sealed {}
    impl Sealed for OsStrItem {}
}

/// Create an [`OsString`] from a single OS string item
pub trait OsStringFromItem: os_string_from_item_sealed::Sealed {
    /// Create an [`OsString`] from a single OS string item
    fn to_os_string(self) -> OsString;
}

impl OsStringFromItem for OsStrItem {
    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn to_os_string(self) -> OsString {
        OsStr::from_bytes(&[self]).to_os_string()
    }

    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    fn to_os_string(self) -> OsString {
        OsString::from_wide(&[self])
    }

    #[cfg(doc)]
    fn to_os_string(self) -> OsString {
        unreachable!()
    }
}

mod os_str_manip_sealed {
    pub trait Sealed {}
    impl Sealed for std::ffi::OsStr {}
}

pub trait OsStrManip: os_str_manip_sealed::Sealed {
    fn items(&self) -> OsStrItems<'_>;
    fn index(&self, idx: impl OsStrIndex) -> OsString;
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_unchecked(&self, idx: impl OsStrIndex) -> OsString;
    fn starts_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    fn ends_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    fn contains<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    fn strip_prefix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString>;
    fn strip_suffix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString>;
}

impl OsStrManip for OsStr {
    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn items(&self) -> OsStrItems<'_> {
        OsStrItems(self.as_bytes().iter().copied())
    }
    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    fn items(&self) -> OsStrItems<'_> {
        OsStrItems(self.encode_wide())
    }
    #[cfg(doc)]
    fn items(&self) -> OsStrItems<'_> {
        unreachable!()
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
    fn contains<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool {
        pat.is_contained_in(self)
    }
    fn strip_prefix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString> {
        pat.strip_prefix_of(self)
    }
    fn strip_suffix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString> {
        pat.strip_suffix_of(self)
    }
}

mod os_str_index_sealed {
    pub trait Sealed {}
    impl Sealed for std::ops::Range<usize> {}
    impl Sealed for std::ops::RangeFrom<usize> {}
    impl Sealed for std::ops::RangeFull {}
    impl Sealed for std::ops::RangeInclusive<usize> {}
    impl Sealed for std::ops::RangeTo<usize> {}
    impl Sealed for std::ops::RangeToInclusive<usize> {}
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

mod os_string_from_iter_sealed {
    use super::OsStrItem;

    pub trait Sealed {}
    impl<T: Iterator<Item = OsStrItem>> Sealed for T {}
}

pub trait OsStringFromIter:
    Iterator<Item = OsStrItem> + os_string_from_iter_sealed::Sealed + Sized
{
    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn to_os_string(self) -> OsString {
        OsStr::from_bytes(&self.collect::<OsStrVec>()).to_os_string()
    }
    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    fn to_os_string(self) -> OsString {
        OsString::from_wide(&self.collect::<OsStrVec>())
    }
    #[cfg(doc)]
    fn to_os_string(self) -> OsString {
        unreachable!()
    }
}

impl<T: Iterator<Item = OsStrItem>> OsStringFromIter for T {}

#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone, Debug)]
pub struct OsStrItems<'a>(std::iter::Copied<std::slice::Iter<'a, OsStrItem>>);

#[cfg(target_family = "windows")]
#[derive(Clone, Debug)]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;

    #[cfg(not(doc))]
    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?)
    }
    #[cfg(not(doc))]
    #[cfg(target_family = "windows")]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?)
    }
    #[cfg(doc)]
    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

mod os_str_pattern_sealed {
    pub trait Sealed {}
    impl Sealed for super::OsStrItem {}
    impl<C: super::OsStrMultiItemEq> Sealed for C {}
    impl Sealed for &std::ffi::OsStr {}
    impl Sealed for &std::ffi::OsString {}
}

pub trait OsStrPattern<'a>: os_str_pattern_sealed::Sealed + Sized {
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

mod os_str_searcher_sealed {
    pub trait Sealed {}
    impl Sealed for super::OsStrItemSearcher<'_> {}
    impl<C: super::OsStrMultiItemEq> Sealed for super::OsStrMultiItemEqSearcher<'_, C> {}
    impl Sealed for super::OsStrSubstringSearcher<'_, '_> {}
}

pub trait OsStrSearcher: os_str_searcher_sealed::Sealed {
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

#[derive(Clone, Debug)]
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
            needle: self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OsStrItemSearcher<'a> {
    haystack: OsStrItems<'a>,
    finger: usize,
    needle: OsStrItem,
}

impl OsStrSearcher for OsStrItemSearcher<'_> {
    fn next(&mut self) -> OsStrSearchStep {
        let result = match self.haystack.next() {
            Some(item) if item == self.needle => {
                self.finger += 1;
                OsStrSearchStep::Match(self.finger, self.finger + 1)
            }
            Some(_) => {
                self.finger += 1;
                OsStrSearchStep::Reject(self.finger, self.finger + 1)
            }
            None => OsStrSearchStep::Done,
        };
        result
    }
}

mod os_str_multi_item_eq_sealed {
    use super::OsStrItem;

    pub trait Sealed {}
    impl<F: FnMut(OsStrItem) -> bool> Sealed for F {}
    impl<const N: usize> Sealed for [OsStrItem; N] {}
    impl<const N: usize> Sealed for &[OsStrItem; N] {}
    impl Sealed for &[OsStrItem] {}
}

pub trait OsStrMultiItemEq: os_str_multi_item_eq_sealed::Sealed {
    fn matches(&mut self, item: OsStrItem) -> bool;
}

impl<F: FnMut(OsStrItem) -> bool> OsStrMultiItemEq for F {
    fn matches(&mut self, item: OsStrItem) -> bool {
        self(item)
    }
}

impl<const N: usize> OsStrMultiItemEq for [OsStrItem; N] {
    fn matches(&mut self, item: OsStrItem) -> bool {
        self.contains(&item)
    }
}

impl<const N: usize> OsStrMultiItemEq for &[OsStrItem; N] {
    fn matches(&mut self, item: OsStrItem) -> bool {
        self.contains(&item)
    }
}

impl OsStrMultiItemEq for &[OsStrItem] {
    fn matches(&mut self, item: OsStrItem) -> bool {
        self.contains(&item)
    }
}

#[derive(Clone, Debug)]
pub struct OsStrMultiItemEqSearcher<'a, C: OsStrMultiItemEq> {
    haystack: OsStrItems<'a>,
    finger: usize,
    needle: C,
}

impl<'a, C: OsStrMultiItemEq> OsStrMultiItemEqSearcher<'a, C> {
    fn new(haystack: OsStrItems<'a>, needle: C) -> Self {
        Self {
            haystack,
            finger: 0,
            needle,
        }
    }
}

impl<C: OsStrMultiItemEq> OsStrSearcher for OsStrMultiItemEqSearcher<'_, C> {
    fn next(&mut self) -> OsStrSearchStep {
        let result = match self.haystack.next() {
            Some(item) if self.needle.matches(item) => {
                self.finger += 1;
                OsStrSearchStep::Match(self.finger, self.finger + 1)
            }
            Some(_) => {
                self.finger += 1;
                OsStrSearchStep::Reject(self.finger, self.finger + 1)
            }
            None => OsStrSearchStep::Done,
        };
        result
    }
}

impl<'a, C: OsStrMultiItemEq> OsStrPattern<'a> for C {
    type Searcher = OsStrMultiItemEqSearcher<'a, C>;

    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher {
        Self::Searcher::new(haystack.items(), self)
    }
}

#[derive(Clone, Debug)]
pub struct OsStrSubstringSearcher<'a, 'b> {
    haystack: &'a OsStr,
    finger: usize,
    details: OsStrSubstringSearcherImpl<'b>,
}

#[derive(Clone, Debug)]
enum OsStrSubstringSearcherImpl<'a> {
    NonEmptyNeedle { needle: &'a OsStr },
    EmptyNeedle { finished: bool },
}

impl<'a, 'b> OsStrSubstringSearcher<'a, 'b> {
    fn new(haystack: &'a OsStr, needle: &'b OsStr) -> Self {
        Self {
            haystack,
            finger: 0,
            details: if needle.is_empty() {
                OsStrSubstringSearcherImpl::EmptyNeedle { finished: false }
            } else {
                OsStrSubstringSearcherImpl::NonEmptyNeedle { needle }
            },
        }
    }
}

impl<'a, 'b> OsStrSearcher for OsStrSubstringSearcher<'a, 'b> {
    fn next(&mut self) -> OsStrSearchStep {
        match self.details {
            OsStrSubstringSearcherImpl::EmptyNeedle { ref mut finished } => {
                if *finished {
                    OsStrSearchStep::Done
                } else {
                    let start = self.finger;
                    if self.finger == self.haystack.len() {
                        *finished = true;
                    } else {
                        self.finger += 1;
                    }
                    OsStrSearchStep::Match(start, start)
                }
            }
            OsStrSubstringSearcherImpl::NonEmptyNeedle { needle } => {
                let len = self.haystack.len();
                let start = self.finger;
                if self.finger == len {
                    OsStrSearchStep::Done
                } else if self.haystack.len() - start < needle.len() {
                    self.finger = len;
                    OsStrSearchStep::Reject(start, self.haystack.len())
                } else {
                    let mut needle_iter = needle.items();
                    for (haystack_item, needle_item) in
                        self.haystack.items().skip(start).zip(needle_iter.by_ref())
                    {
                        self.finger += 1;
                        if haystack_item != needle_item {
                            return OsStrSearchStep::Reject(start, self.finger);
                        }
                    }
                    debug_assert!(needle_iter.next().is_none());
                    OsStrSearchStep::Match(start, self.finger)
                }
            }
        }
    }
}

impl<'a, 'b> OsStrPattern<'a> for &'b OsStr {
    type Searcher = OsStrSubstringSearcher<'a, 'b>;

    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher {
        Self::Searcher::new(haystack, self)
    }
}

impl<'a, 'b> OsStrPattern<'a> for &'b OsString {
    type Searcher = OsStrSubstringSearcher<'a, 'b>;

    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher {
        Self::Searcher::new(haystack, self)
    }
}
