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

/// This type is used throughout this crate’s documentation to refer
/// to the items an [`OsStr`] can be decomposed into and constructed from
///
/// On Windows, this is a [`u16`]
///
/// On Unix and WASI, this is a [`u8`]
///
/// [`OsStr`]: std::ffi::OsStr
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
///
/// This trait is sealed, it cannot be implemented for any additional types
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

/// Various string manipulation methods for [`OsStr`], and, by [`Deref`], [`OsString`]
///
/// This trait is sealed, it cannot be implemented for any additional types
///
/// [`Deref`]: std::ops::Deref
pub trait OsStrManip: os_str_manip_sealed::Sealed {
    /// Get an iterator over the items a string consists of
    fn items(&self) -> OsStrItems<'_>;
    /// Construct a substring or get an item by index or range
    ///
    /// Note that when constructing a substring, this method constructs
    /// a new, owned, [`OsString`], due to platform limitations
    ///
    /// # Panics
    ///
    /// When `idx` is out of bounds of the string
    /// (For ranges, this means that any component is out of bounds)
    ///
    /// When `idx` is a range and its (inclusive) lower bound is above its
    /// (inclusive) upper bound
    fn index<T: OsStrIndex>(&self, idx: T) -> T::Output;
    #[cfg(feature = "unchecked_index")]
    /// This method requires the feature `unchecked_index` and nightly rust due to
    /// relying on unstable features
    ///
    /// Like [`index`], but instead of panicking, it causes undefined behavior
    ///
    /// Note that when constructing a substring, this method constructs
    /// a new, owned, [`OsString`], due to platform limitations
    ///
    /// # Safety
    ///
    /// It is **required**:
    /// - If `idx: Range<usize>`:
    ///     - `idx.start <= idx.end`
    ///     - `idx.start <= self.len()`
    ///     - `idx.end <= self.len()`
    /// - If `idx: RangeFrom<usize>` that `idx.start <= self.len()`
    /// - If `idx: RangeInclusive<usize>`:
    ///     - `*idx.start() <= *idx.end() + 1`
    ///     - `*idx.start() <= self.len()`
    ///     - `*idx.end() < self.len()`
    /// - If `idx: RangeTo<usize>` that `idx.end <= self.len()`
    /// - If `idx: RangeToInclusive<usize>` that `idx.end < self.len()`
    /// - If `idx: usize` that `idx < self.len()`
    /// - If `idx: RangeFull` then nothing is required
    ///
    /// [`index`]: OsStrManip::index
    unsafe fn index_unchecked(&self, idx: impl OsStrIndex) -> OsString;
    /// Check if an [`OsStr`] starts with a pattern
    ///
    /// # Examples
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// assert!(OsStr::new("Hello, world!").starts_with(OsStr::new("Hell")));
    /// assert!(!OsStr::new("Lucifer").starts_with(OsStr::new("Money")));
    /// assert!(OsStr::new("Opportunism").starts_with(OsStr::new("")));
    /// assert!(!OsStr::new("Ferris").starts_with(OsStr::new("Ferris wheel")));
    /// assert!(OsStr::new("Howl").starts_with(OsStr::new("Howl")));
    /// ```
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// let str = OsStr::new("Optometrist");
    /// assert!(str.starts_with(str.items().nth(0).unwrap()))
    /// ```
    fn starts_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    /// Check if an [`OsStr`] string ends with a pattern
    ///
    /// # Examples
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// assert!(OsStr::new("Sputnik").ends_with(OsStr::new("nik")));
    /// assert!(!OsStr::new("Opera").ends_with(OsStr::new("Oxen")));
    /// assert!(OsStr::new("Failure").ends_with(OsStr::new("")));
    /// assert!(!OsStr::new("Cloak").ends_with(OsStr::new("Cloaked in shadow")));
    /// assert!(OsStr::new("Moped").ends_with(OsStr::new("Moped")));
    /// ```
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// let str = OsStr::new("Pipes");
    /// assert!(str.ends_with(str.items().last().unwrap()))
    /// ```
    fn ends_with<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    /// Check if an [`OsStr`] string contains a pattern
    ///
    /// # Examples
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// assert!(OsStr::new("Coca-Cola").contains(OsStr::new("-")));
    /// assert!(OsStr::new("Comedy").contains(OsStr::new("ome")));
    /// assert!(OsStr::new("Plethora").contains(OsStr::new("Plethora")));
    /// assert!(!OsStr::new("Oak").contains(OsStr::new("y")));
    /// assert!(!OsStr::new("Bayesian").contains(OsStr::new("Bayesian inference")));
    /// assert!(OsStr::new("Freeze").contains(OsStr::new("")));
    /// ```
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// let str = OsStr::new("Idempotency");
    /// assert!(str.contains(str.items().nth(4).unwrap()));
    /// ```
    fn contains<'a>(&'a self, pat: impl OsStrPattern<'a>) -> bool;
    /// Remove the prefix matching a pattern from the start of an [`OsStr`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// assert_eq!(OsStr::new("Union").strip_prefix(OsStr::new("Un")).as_deref(), Some(OsStr::new("ion")));
    /// assert_eq!(OsStr::new("Cape").strip_prefix(OsStr::new("pe")).as_deref(), None);
    /// assert_eq!(OsStr::new("Flake").strip_prefix(OsStr::new("xy")).as_deref(), None);
    /// assert_eq!(OsStr::new("Human").strip_prefix(OsStr::new("Humanity")).as_deref(), None);
    /// assert_eq!(OsStr::new("Grapefruit").strip_prefix(OsStr::new("Grapefruit")).as_deref(), Some(OsStr::new("")));
    /// ```
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// let str = OsStr::new("Hors d'oeuvre");
    /// assert_eq!(str.strip_prefix(str.items().nth(0).unwrap()).as_deref(), Some(OsStr::new("ors d'oeuvre")));
    /// ```
    fn strip_prefix<'a>(&'a self, pat: impl OsStrPattern<'a>) -> Option<OsString>;
    /// Remove the suffix matching a pattern from the end of an [`OsStr`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// assert_eq!(OsStr::new("Globe").strip_suffix(OsStr::new("be")).as_deref(), Some(OsStr::new("Glo")));
    /// assert_eq!(OsStr::new("Caretaker").strip_suffix(OsStr::new("Ca")).as_deref(), None);
    /// assert_eq!(OsStr::new("Upright").strip_suffix(OsStr::new("foo")).as_deref(), None);
    /// assert_eq!(OsStr::new("Color").strip_suffix(OsStr::new("New Color")).as_deref(), None);
    /// assert_eq!(OsStr::new("Ink").strip_suffix(OsStr::new("Ink")).as_deref(), Some(OsStr::new("")));
    /// ```
    /// ```
    /// # use os_str_manip::os_str_manip::OsStrManip;
    /// # use std::ffi::OsStr;
    /// let str = OsStr::new("Catacombs");
    /// assert_eq!(str.strip_suffix(str.items().last().unwrap()).as_deref(), Some(OsStr::new("Catacomb")));
    /// ```
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
    fn index<T: OsStrIndex>(&self, idx: T) -> T::Output {
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

/// Get a part of an `&`[`OsStr`]
///
/// This trait is used by the [`OsStrManip::index`] function,
/// see there for more information
///
/// This trait is sealed, it cannot be implemented for any additional types
pub trait OsStrIndex {
    type Output;

    /// Get a part of an `&`[`OsStr`]
    ///
    /// For panic information see [`OsStrManip::index`]
    fn index_of(self, source: &OsStr) -> Self::Output;
    /// This method requires the feature `unchecked_index` and nightly rust due to
    /// relying on unstable features
    ///
    /// Get a part of an `&`[`OsStr`], but cause undefined behavior
    /// on out-of-bounds indices
    ///
    /// For exact safety requirements see [`OsStrManip::index_unchecked`]
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output;
}

impl OsStrIndex for std::ops::Range<usize> {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(self.start <= self.end);
        assert!(self.start <= source.len());
        assert!(self.end <= source.len());
        source
            .items()
            .skip(self.start)
            .take(self.end - self.start)
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(self.start <= self.end);
        debug_assert!(self.start <= source.len());
        debug_assert!(self.end <= source.len());
        source
            .items()
            .skip(self.start)
            .take(self.end.unchecked_sub(self.start))
            .to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeFrom<usize> {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(self.start <= source.len());
        source.items().skip(self.start).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(self.start <= source.len());
        source.items().skip(self.start).to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeFull {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        source.to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        source.to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeInclusive<usize> {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(*self.start() <= *self.end() + 1);
        assert!(*self.start() <= source.len());
        assert!(*self.end() < source.len());
        source
            .items()
            .skip(*self.start())
            .take(self.end() + 1 - self.start())
            .to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(*self.start() <= *self.end() + 1);
        debug_assert!(*self.start() <= source.len());
        debug_assert!(*self.end() < source.len());
        source
            .items()
            .skip(*self.start())
            .take(self.end().unchecked_add(1).unchecked_sub(*self.start()))
            .to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeTo<usize> {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(self.end <= source.len());
        source.items().take(self.end).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(self.end <= source.len());
        source.items().take(self.end).to_os_string()
    }
}

impl OsStrIndex for std::ops::RangeToInclusive<usize> {
    type Output = OsString;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(self.end < source.len());
        source.items().take(self.end + 1).to_os_string()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(self.end < source.len());
        source
            .items()
            .take(self.end.unckeched_add(1))
            .to_os_string()
    }
}

impl OsStrIndex for usize {
    type Output = OsStrItem;

    fn index_of(self, source: &OsStr) -> Self::Output {
        assert!(self < source.len());
        source.items().nth(self).unwrap()
    }
    #[cfg(feature = "unchecked_index")]
    unsafe fn index_of_unchecked(self, source: &OsStr) -> Self::Output {
        debug_assert!(self < source.len());
        source.items().nth(self).unwrap_unchecked()
    }
}

mod os_string_from_iter_sealed {
    use super::OsStrItem;

    pub trait Sealed {}
    impl<T: Iterator<Item = OsStrItem>> Sealed for T {}
}

/// Collect an iterator over items of an [`OsStr`],into an new, owned, [`OsString`]
///
/// This trait is sealed, it cannot be implemented for any additional types
pub trait OsStringFromIter:
    Iterator<Item = OsStrItem> + os_string_from_iter_sealed::Sealed + Sized
{
    /// Collect an iterator over items of an [`OsStr`] into an new, owned, [`OsString`]
    ///
    /// # Example
    ///
    /// ```
    /// # use os_str_manip::os_str_manip::OsStringFromIter;
    /// assert_eq!(OsStr::new("Puppet").items().to_os_string(), OsStr::new("Puppet").to_os_string());
    /// ```
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

#[cfg(not(doc))]
#[cfg(any(target_os = "wasi", target_family = "unix"))]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::iter::Copied<std::slice::Iter<'a, OsStrItem>>);

#[cfg(target_family = "windows")]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

/// Iterator over items of an [`OsStr`], obtained by [`OsStrManip::items`]
#[cfg(doc)]
#[derive(Clone)]
pub struct OsStrItems<'a>(std::iter::Once<OsStrItem>, &'a ());

impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;

    #[cfg(any(target_os = "wasi", target_family = "unix"))]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?)
    }
    #[cfg(target_family = "windows")]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?)
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

/// A pattern for searching in [`OsStr`]s
///
/// This is similar to [`std::str::pattern::Pattern`]
///
/// Each pattern has a [`Searcher`] type, which handles the state
/// of the search process, which can be constructed with a “haystack”,
/// or `&`[`OsStr`] to search in, using [`into_searcher`]
///
/// The meanings of the implementers are:
/// - Searching for an item of an [`OsStr`] checks for any occurrence of the item
/// - Searching for a slice or array of items checks for any occurrence of any of the items
/// - Searching for a function taking items and returning booleans checks for any items that match the predicate
/// - Searching for an `&`[`OsStr`] or `&`[`OsString`] searches for any substring occurrence
///
/// This trait is sealed, it cannot be implemented for any additional types
///
/// [`Searcher`]: OsStrPattern::Searcher
/// [`into_searcher`]: OsStrPattern::into_searcher
pub trait OsStrPattern<'a>: os_str_pattern_sealed::Sealed + Sized {
    type Searcher: OsStrSearcher;

    /// Construct the searcher for a given `&`[`OsStr`] to search
    fn into_searcher(self, haystack: &'a OsStr) -> Self::Searcher;

    /// Check if an [`OsStr`] contains a pattern
    fn is_contained_in(self, haystack: &'a OsStr) -> bool {
        self.into_searcher(haystack).next_match().is_some()
    }
    /// Check if an [`OsStr`] starts with a pattern
    fn is_prefix_of(self, haystack: &'a OsStr) -> bool {
        matches!(
            self.into_searcher(haystack).next(),
            OsStrSearchStep::Match(0, _)
        )
    }
    /// Check if an [`OsStr`] ends with a pattern
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
    /// Remove the prefix matching a pattern from the start of an [`OsStr`]
    fn strip_prefix_of(self, haystack: &'a OsStr) -> Option<OsString> {
        if let OsStrSearchStep::Match(start, len) = self.into_searcher(haystack).next() {
            debug_assert_eq!(start, 0, "OsStrSearcher::next().0 must be 0 on first call");
            Some(haystack.index(len..))
        } else {
            None
        }
    }
    /// Remove the suffix matching a pattern from the end of an [`OsStr`]
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

/// A searcher that encapsulates the state of the search for
/// an [`OsStrPattern`]
///
/// Calling [`next`] repeatedly will produce adjacent ranges
/// of substrings that either match the pattern or cannot be part of a match,
/// followed by [`OsStrSearchStep::Done`] when the `&`[`OsStr`]’s end is reached
///
/// [`next`]: OsStrSearcher::next
pub trait OsStrSearcher: os_str_searcher_sealed::Sealed {
    /// Get the next fully processed substring range and its judgement
    fn next(&mut self) -> OsStrSearchStep;

    /// Get the next fully processed substring range that was judged to be a match
    fn next_match(&mut self) -> Option<(usize, usize)> {
        loop {
            match self.next() {
                OsStrSearchStep::Match(a, b) => return Some((a, b)),
                OsStrSearchStep::Done => return None,
                _ => continue,
            }
        }
    }
    /// Get the next fully processed substring range that was rejected
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

/// A search step produced by [`OsStrSearcher::next`]
///
/// The indices here are left inclusive, right exclusive
#[derive(Clone, Debug)]
pub enum OsStrSearchStep {
    /// A subrange was identified as a match
    Match(usize, usize),
    /// A subrange was ruled out as being part of a match
    Reject(usize, usize),
    /// The searched `&`[`OsStr`] was exhausted
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

#[derive(Clone)]
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
                OsStrSearchStep::Match(self.finger - 1, self.finger)
            }
            Some(_) => {
                self.finger += 1;
                OsStrSearchStep::Reject(self.finger - 1, self.finger)
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

#[derive(Clone)]
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
