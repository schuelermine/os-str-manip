use std::ffi::OsStr;

#[cfg(target_family = "windows")]
pub struct OsStrItem(u16);

#[cfg(any(target_family = "wasm", target_family = "unix"))]
#[derive(Clone, Copy)]
pub struct OsStrItem(u8);

pub trait OsStrManip {
    fn iter_items<'a>(&'a self) -> OsStrItems<'a>;
    fn contains<'a, P: OsPattern<'a>>(&'a self, pat: P) -> bool;
}

#[cfg(target_family = "windows")]
pub struct OsStrItems<'a>(std::os::windows::ffi::EncodeWide<'a>);

#[cfg(any(target_family = "wasm", target_family = "unix"))]
pub struct OsStrItems<'a>(std::slice::Iter<'a, u8>);

#[cfg(target_family = "windows")]
impl<'a> Iterator for OsStrItem<'a> {
    type Item = OsStrItem;
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(self.0.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[cfg(any(target_family = "wasm", target_family = "unix"))]
impl<'a> Iterator for OsStrItems<'a> {
    type Item = OsStrItem;
    fn next(&mut self) -> Option<Self::Item> {
        Some(OsStrItem(*self.0.next()?))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub trait OsPattern<'a> {
    fn matches(&self, haystack: &'a OsStr) -> Option<usize>;
}
