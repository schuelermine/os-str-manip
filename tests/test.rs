use os_str_manip::os_str_manip::*;
use proptest::prelude::*;

#[cfg(any(target_os = "wasi", target_family = "unix"))]
use std::ffi::{OsStr, OsString};

#[cfg(target_family = "windows")]
use std::ffi::OsString;

#[cfg(any(target_os = "wasi", target_family = "unix"))]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStringExt;

#[cfg(any(target_os = "wasi", target_family = "unix"))]
type OsStrItem = u8;

#[cfg(target_family = "windows")]
type OsStrItem = u16;

const ITEM_STRATEGY: std::ops::RangeInclusive<OsStrItem> = OsStrItem::MIN..=OsStrItem::MAX;
const SIZE_RANGE: std::ops::RangeInclusive<usize> = 1..=10;

#[cfg(any(target_os = "wasi", target_family = "unix"))]
fn os_string_strategy(
    size: impl Into<proptest::collection::SizeRange>,
) -> impl Strategy<Value = OsString> {
    proptest::collection::vec(ITEM_STRATEGY, size)
        .prop_map(|v| OsStr::from_bytes(&v).to_os_string())
}

#[cfg(target_family = "windows")]
fn os_string_strategy(
    size: impl Into<proptest::collection::SizeRange>,
) -> impl Strategy<Value = OsString> {
    proptest::collection::vec(ITEM_STRATEGY, size).prop_map(|v| OsString::from_wide(&v))
}

fn os_string_with_range_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, std::ops::Range<usize>)> {
    size.prop_flat_map(|size| {
        (
            os_string_strategy(size),
            (0..=size).prop_flat_map(move |start| (start..=size).prop_map(move |end| start..end)),
        )
    })
}

fn os_string_with_range_from_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, std::ops::RangeFrom<usize>)> {
    size.prop_flat_map(|size| {
        (
            os_string_strategy(size),
            (0..=size).prop_map(|start| start..),
        )
    })
}

fn os_string_with_range_inclusive_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, std::ops::RangeInclusive<usize>)> {
    size.prop_filter(
        "zero length means inclusive upper bound cannot exist",
        |size| *size != 0,
    )
    .prop_flat_map(|size| {
        (
            os_string_strategy(size),
            (0..=size).prop_flat_map(move |start| {
                (start.saturating_sub(1)..size).prop_map(move |end| start..=end)
            }),
        )
    })
}

fn os_string_with_range_to_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, std::ops::RangeTo<usize>)> {
    size.prop_flat_map(|size| (os_string_strategy(size), (0..=size).prop_map(|end| ..end)))
}

fn os_string_with_range_to_inclusive_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, std::ops::RangeToInclusive<usize>)> {
    size.prop_filter(
        "zero length means inclusive upper bound cannot exist",
        |size| *size != 0,
    )
    .prop_flat_map(|size| (os_string_strategy(size), (0..size).prop_map(|end| ..=end)))
}

fn os_string_with_index_strategy(
    size: impl Strategy<Value = usize>,
) -> impl Strategy<Value = (OsString, usize)> {
    size.prop_filter("zero length means a valid index is impossible", |size| {
        *size != 0
    })
    .prop_flat_map(|size| (os_string_strategy(size), 0..size))
}

proptest! {
    #[test]
    fn singleton_os_strings_agree(item in ITEM_STRATEGY) {
        prop_assert_eq!(item.to_os_string(), std::iter::once(item).to_os_string());
    }

    #[test]
    fn range_indexing_agrees_with_singleton_from_item_indexing(
        (string, index) in os_string_with_index_strategy(SIZE_RANGE)
    ) {
        prop_assert_eq!(string.index(index..=index), string.index(index).to_os_string());
        prop_assert_eq!(string.index(index..index + 1), string.index(index).to_os_string());
    }

    #[test]
    fn items_are_elements(string in os_string_strategy(SIZE_RANGE)) {
        prop_assert!(string.items().all(|item| string.contains(item)));
    }

    #[test]
    fn item_indexing_produces_elements(
        (string, index) in os_string_with_index_strategy(SIZE_RANGE)
    ) {
        prop_assert!(string.index(index).is_contained_in(&string));
    }

    #[test]
    fn test_range_index(
        (string, range) in os_string_with_range_strategy(SIZE_RANGE)
    ) {
        let substring = string.index(range.clone());
        prop_assert_eq!(substring.items().count(), range.end - range.start);
        prop_assert!(substring.items().all(|item| string.contains(item)));
        prop_assert!(substring.items().enumerate().all(|(index, item)| string.index(range.start + index) == item));
    }

    #[test]
    fn test_range_from_index(
        (string, range) in os_string_with_range_from_strategy(SIZE_RANGE)
    ) {
        let substring = string.index(range.clone());
        prop_assert_eq!(substring.items().count(), string.items().count() - range.start);
        prop_assert!(substring.items().all(|item| string.contains(item)));
        prop_assert!(substring.items().enumerate().all(|(index, item)| string.index(range.start + index) == item));
    }

    #[test]
    fn test_range_full_index(string in os_string_strategy(SIZE_RANGE)) {
        let new_string = string.index(..);
        prop_assert_eq!(&string, &new_string);
    }

    #[test]
    fn test_range_inclusive_index(
        (string, range) in os_string_with_range_inclusive_strategy(SIZE_RANGE)
    ) {
        let substring = string.index(range.clone());
        prop_assert_eq!(substring.items().count(), range.end() + 1 - range.start());
        prop_assert!(substring.items().all(|item| string.contains(item)));
        prop_assert!(substring.items().enumerate().all(|(index, item)| string.index(range.start() + index) == item));
    }

    #[test]
    fn test_range_to_index(
        (string, range) in os_string_with_range_to_strategy(SIZE_RANGE)
    ) {
        let substring = string.index(range);
        prop_assert_eq!(substring.items().count(), range.end);
        prop_assert!(substring.items().all(|item| string.contains(item)));
        prop_assert!(substring.items().enumerate().all(|(index, item)| string.index(index) == item));
    }

    #[test]
    fn test_range_to_inclusive_index(
        (string, range) in os_string_with_range_to_inclusive_strategy(SIZE_RANGE)
    ) {
        let substring = string.index(range);
        prop_assert_eq!(substring.items().count(), range.end + 1);
        prop_assert!(substring.items().all(|item| string.contains(item)));
        prop_assert!(substring.items().enumerate().all(|(index, item)| string.index(index) == item));
    }
}
