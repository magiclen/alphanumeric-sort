/*!
# Alphanumeric Sort

This crate can help you sort order for files and folders whose names contain numerals.

## Motives and Examples

With the Rust native `sort` method, strings and paths are arranged into lexicographical order. In some cases, it is not so intuitive. For example, there are screen snap shots named by **shot-%N** like **shot-2**, **shot-1**, **shot-11**. After a lexicographical sorting, they will be ordered into **shot-1**, **shot-11**, **shot-2**. However, we would prefer **shot-1**, **shot-2**, **shot-11** mostly.

```rust
let mut names = ["shot-2", "shot-1", "shot-11"];

names.sort();

assert_eq!(["shot-1", "shot-11", "shot-2"], names);
```

Thus, in this kind of case, an alphanumeric sort might come in handy.

```rust
extern crate alphanumeric_sort;

let mut names = ["shot-2", "shot-1", "shot-11"];

alphanumeric_sort::sort_str_slice(&mut names);

assert_eq!(["shot-1", "shot-2", "shot-11"], names);
```

```rust
extern crate alphanumeric_sort;

# #[cfg(feature = "std")] {
use std::path::Path;

let mut paths = [Path::new("shot-2"), Path::new("shot-1"), Path::new("shot-11")];

alphanumeric_sort::sort_path_slice(&mut paths);

assert_eq!([Path::new("shot-1"), Path::new("shot-2"), Path::new("shot-11")], paths);
# }
```

## About the `compare_*` Functions and the `sort_*` Functions

To sort a slice, the code can also be written like,

```rust
extern crate alphanumeric_sort;

# #[cfg(feature = "std")] {
use std::path::Path;

let mut paths = [Path::new("shot-2"), Path::new("shot-1"), Path::new("shot-11")];

paths.sort_by(|a, b| alphanumeric_sort::compare_path(a, b));

assert_eq!([Path::new("shot-1"), Path::new("shot-2"), Path::new("shot-11")], paths);
# }
```

But it is not recommended because the `compare_*` functions try to convert data (e.g `Path`, `CStr`) to `&str` every time in its execution and thus they are slower than the `sort_*` functions when sorting a slice.

## Version `1.3` to `1.4`

No breaking change in API is made, though the order has some changes.

* `"0001"` is greater than `"001"` instead of being equal.
* `"中"` is greater than `"1"` instead of being less. `"第1章"` is still less than `"第1-2章"`, even though `"章"` is greater than `"-"`.

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.alphanumeric-sort]
version = "*"
default-features = false
```

## Benchmark

```bash
cargo bench
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use core::cmp::Ordering;

#[cfg(feature = "std")]
use std::ffi::{CStr, OsStr};

#[cfg(feature = "std")]
use std::path::Path;

#[allow(clippy::while_let_on_iterator)]
/// Compare two strings.
pub fn compare_str<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> Ordering {
    let mut c1 = a.as_ref().chars();
    let mut c2 = b.as_ref().chars();

    // this flag is to handle something like "1點" < "1-1點"
    let mut last_is_number = false;

    let mut v1: Option<char> = None;
    let mut v2: Option<char> = None;

    loop {
        let ca = {
            match v1.take() {
                Some(c) => c,
                None => {
                    match c1.next() {
                        Some(c) => c,
                        None => {
                            if v2.take().is_some() || c2.next().is_some() {
                                return Ordering::Less;
                            } else {
                                return Ordering::Equal;
                            }
                        }
                    }
                }
            }
        };

        let cb = {
            match v2.take() {
                Some(c) => c,
                None => {
                    match c2.next() {
                        Some(c) => c,
                        None => {
                            return Ordering::Greater;
                        }
                    }
                }
            }
        };

        if ('0'..='9').contains(&ca) && ('0'..='9').contains(&cb) {
            let mut da = f64::from(ca as u32) - f64::from(b'0');
            let mut db = f64::from(cb as u32) - f64::from(b'0');

            // this counter is to handle something like "001" > "01"
            let mut dc = 0isize;

            while let Some(ca) = c1.next() {
                if ('0'..='9').contains(&ca) {
                    da = da * 10.0 + (f64::from(ca as u32) - f64::from(b'0'));
                    dc += 1;
                } else {
                    v1 = Some(ca);
                    break;
                }
            }

            while let Some(cb) = c2.next() {
                if ('0'..='9').contains(&cb) {
                    db = db * 10.0 + (f64::from(cb as u32) - f64::from(b'0'));
                    dc -= 1;
                } else {
                    v2 = Some(cb);
                    break;
                }
            }

            last_is_number = true;

            match da.partial_cmp(&db) {
                Some(ordering) if ordering != Ordering::Equal => {
                    return ordering;
                }
                _ => {
                    match dc.cmp(&0) {
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    }
                }
            }
        } else {
            match ca.cmp(&cb) {
                Ordering::Equal => last_is_number = false,
                Ordering::Greater => {
                    return if last_is_number && (ca > (255 as char)) ^ (cb > (255 as char)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }
                Ordering::Less => {
                    return if last_is_number && (ca > (255 as char)) ^ (cb > (255 as char)) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                }
            }
        }
    }
}

/// Compare two `OsStr`.
#[cfg(feature = "std")]
#[inline]
pub fn compare_os_str<A: AsRef<OsStr>, B: AsRef<OsStr>>(a: A, b: B) -> Ordering {
    let sa = match a.as_ref().to_str() {
        Some(s) => s,
        None => {
            return compare_os_str_fallback(a, b);
        }
    };

    let sb = match b.as_ref().to_str() {
        Some(s) => s,
        None => {
            return compare_os_str_fallback(a, b);
        }
    };

    compare_str(sa, sb)
}

#[cfg(feature = "std")]
#[inline]
fn compare_os_str_fallback<A: AsRef<OsStr>, B: AsRef<OsStr>>(a: A, b: B) -> Ordering {
    a.as_ref().cmp(b.as_ref())
}

/// Compare two `CStr`.
#[cfg(feature = "std")]
#[inline]
pub fn compare_c_str<A: AsRef<CStr>, B: AsRef<CStr>>(a: A, b: B) -> Ordering {
    let sa = match a.as_ref().to_str() {
        Ok(s) => s,
        Err(_) => {
            return compare_c_str_fallback(a, b);
        }
    };

    let sb = match b.as_ref().to_str() {
        Ok(s) => s,
        Err(_) => {
            return compare_c_str_fallback(a, b);
        }
    };

    compare_str(sa, sb)
}

#[cfg(feature = "std")]
#[inline]
fn compare_c_str_fallback<A: AsRef<CStr>, B: AsRef<CStr>>(a: A, b: B) -> Ordering {
    a.as_ref().cmp(b.as_ref())
}

/// Compare two `Path`.
#[cfg(feature = "std")]
#[inline]
pub fn compare_path<A: AsRef<Path>, B: AsRef<Path>>(a: A, b: B) -> Ordering {
    compare_os_str(a.as_ref(), b.as_ref())
}

// TODO -----------

/// Sort a slice by a `str` key, but may not preserve the order of equal elements.
#[inline]
pub fn sort_slice_unstable_by_str_key<A, T: ?Sized + AsRef<str>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_str(f(a), f(b)));
}

/// Sort a slice by a `str` key.
#[inline]
pub fn sort_slice_by_str_key<A, T: ?Sized + AsRef<str>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_str(f(a), f(b)));
}

/// Reversely sort a slice by a `str` key, but may not preserve the order of equal elements.
#[inline]
pub fn sort_slice_rev_unstable_by_str_key<A, T: ?Sized + AsRef<str>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_str(f(b), f(a)));
}

/// Reversely sort a slice by a `str` key.
#[inline]
pub fn sort_slice_rev_by_str_key<A, T: ?Sized + AsRef<str>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_str(f(b), f(a)));
}

/// Sort a slice by an `OsStr` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_unstable_by_os_str_key<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_os_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_unstable,
        sort_slice_unstable_by_os_str_key_fallback,
    )
}

/// Sort a slice by an `OsStr` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_by_os_str_key<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_os_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes,
        sort_slice_by_os_str_key_fallback,
    )
}

/// Reversely sort a slice by an `OsStr` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_unstable_by_os_str_key<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_os_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev_unstable,
        sort_slice_rev_unstable_by_os_str_key_fallback,
    )
}

/// Reversely sort a slice by an `OsStr` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_by_os_str_key<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_os_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev,
        sort_slice_rev_by_os_str_key_fallback,
    )
}

#[cfg(feature = "std")]
fn sort_slice_by_os_str_key_inner<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
    ref_index_str_pairs_to_ref_indexes: impl Fn(Vec<(usize, &str)>) -> Vec<(usize, usize)>,
    fallback: impl Fn(&mut [A], F),
) {
    let mut use_str = true;

    let mut ref_index_str_pairs = Vec::with_capacity(slice.len());

    for (i, p) in slice.iter().enumerate() {
        let s = match f(p).as_ref().to_str() {
            Some(s) => s,
            None => {
                use_str = false;
                break;
            }
        };

        ref_index_str_pairs.push((i, s));
    }

    if use_str {
        let ref_indexes = ref_index_str_pairs_to_ref_indexes(ref_index_str_pairs);

        sort_slice_ref_indexes(slice, ref_indexes);
    } else {
        fallback(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_unstable_by_os_str_key_fallback<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_os_str_fallback(f(a), f(b)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_os_str_key_fallback<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_os_str_fallback(f(a), f(b)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_unstable_by_os_str_key_fallback<
    A,
    T: ?Sized + AsRef<OsStr>,
    F: FnMut(&A) -> &T,
>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_os_str_fallback(f(b), f(a)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_by_os_str_key_fallback<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_os_str_fallback(f(b), f(a)));
}

/// Sort a slice by a `CStr` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_unstable_by_c_str_key<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_c_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_unstable,
        sort_slice_unstable_by_c_str_key_fallback,
    )
}

/// Sort a slice by a `CStr` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_by_c_str_key<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_c_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes,
        sort_slice_by_c_str_key_fallback,
    )
}

/// Reversely sort a slice by a `CStr` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_unstable_by_c_str_key<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_c_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev_unstable,
        sort_slice_rev_unstable_by_c_str_key_fallback,
    )
}

/// Reversely sort a slice by a `CStr` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_by_c_str_key<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_c_str_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev,
        sort_slice_rev_by_c_str_key_fallback,
    )
}

#[cfg(feature = "std")]
fn sort_slice_by_c_str_key_inner<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
    ref_index_str_pairs_to_ref_indexes: impl Fn(Vec<(usize, &str)>) -> Vec<(usize, usize)>,
    fallback: impl Fn(&mut [A], F),
) {
    let mut use_str = true;

    let mut ref_index_str_pairs = Vec::with_capacity(slice.len());

    for (i, p) in slice.iter().enumerate() {
        let s = match f(p).as_ref().to_str() {
            Ok(s) => s,
            Err(_) => {
                use_str = false;
                break;
            }
        };

        ref_index_str_pairs.push((i, s));
    }

    if use_str {
        let ref_indexes = ref_index_str_pairs_to_ref_indexes(ref_index_str_pairs);

        sort_slice_ref_indexes(slice, ref_indexes);
    } else {
        fallback(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_unstable_by_c_str_key_fallback<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_c_str_fallback(f(a), f(b)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_c_str_key_fallback<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_c_str_fallback(f(a), f(b)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_unstable_by_c_str_key_fallback<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| compare_c_str_fallback(f(b), f(a)));
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_by_c_str_key_fallback<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_c_str_fallback(f(b), f(a)));
}

/// Sort a slice by a `Path` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_unstable_by_path_key<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_path_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_unstable,
        sort_slice_unstable_by_path_key_fallback,
    )
}

/// Sort a slice by a `Path` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_by_path_key<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_path_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes,
        sort_slice_by_path_key_fallback,
    )
}

/// Reversely sort a slice by a `Path` key, but may not preserve the order of equal elements.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_unstable_by_path_key<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_path_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev_unstable,
        sort_slice_rev_unstable_by_path_key_fallback,
    )
}

/// Reversely sort a slice by a `Path` key.
#[cfg(feature = "std")]
#[inline]
pub fn sort_slice_rev_by_path_key<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    f: F,
) {
    sort_slice_by_path_key_inner(
        slice,
        f,
        ref_index_str_pairs_to_ref_indexes_rev,
        sort_slice_rev_by_path_key_fallback,
    )
}

#[cfg(feature = "std")]
fn sort_slice_by_path_key_inner<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
    ref_index_str_pairs_to_ref_indexes: impl Fn(Vec<(usize, &str)>) -> Vec<(usize, usize)>,
    fallback: impl Fn(&mut [A], F),
) {
    let mut use_str = true;

    let mut ref_index_str_pairs = Vec::with_capacity(slice.len());

    for (i, p) in slice.iter().enumerate() {
        let s = match f(p).as_ref().to_str() {
            Some(s) => s,
            None => {
                use_str = false;
                break;
            }
        };

        ref_index_str_pairs.push((i, s));
    }

    if use_str {
        let ref_indexes = ref_index_str_pairs_to_ref_indexes(ref_index_str_pairs);

        sort_slice_ref_indexes(slice, ref_indexes);
    } else {
        fallback(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_unstable_by_path_key_fallback<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| {
        compare_os_str_fallback(f(a).as_ref().as_os_str(), f(b).as_ref().as_os_str())
    });
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_path_key_fallback<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| {
        compare_os_str_fallback(f(a).as_ref().as_os_str(), f(b).as_ref().as_os_str())
    });
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_unstable_by_path_key_fallback<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_unstable_by(|a, b| {
        compare_os_str_fallback(f(b).as_ref().as_os_str(), f(a).as_ref().as_os_str())
    });
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_rev_by_path_key_fallback<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| {
        compare_os_str_fallback(f(b).as_ref().as_os_str(), f(a).as_ref().as_os_str())
    });
}

// TODO -----------

#[allow(clippy::redundant_closure)]
/// Sort a `str` slice.
#[inline]
pub fn sort_str_slice<S: AsRef<str>>(slice: &mut [S]) {
    slice.sort_unstable_by(|a, b| compare_str(a, b));
}

/// Reversely sort a `str` slice.
#[inline]
pub fn sort_str_slice_rev<S: AsRef<str>>(slice: &mut [S]) {
    slice.sort_unstable_by(|a, b| compare_str(b, a));
}

/// Sort an `OsStr` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_os_str_slice<S: AsRef<OsStr>>(slice: &mut [S]) {
    sort_slice_unstable_by_os_str_key(slice, |e| e.as_ref())
}

/// Reversely sort an `OsStr` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_os_str_slice_rev<S: AsRef<OsStr>>(slice: &mut [S]) {
    sort_slice_rev_unstable_by_os_str_key(slice, |e| e.as_ref())
}

/// Sort a `CStr` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_c_str_slice<S: AsRef<CStr>>(slice: &mut [S]) {
    sort_slice_unstable_by_c_str_key(slice, |e| e.as_ref())
}

/// Reversely sort a `CStr` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_c_str_slice_rev<S: AsRef<CStr>>(slice: &mut [S]) {
    sort_slice_rev_unstable_by_c_str_key(slice, |e| e.as_ref())
}

/// Sort a `Path` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_path_slice<P: AsRef<Path>>(slice: &mut [P]) {
    sort_slice_unstable_by_path_key(slice, |e| e.as_ref())
}

/// Reversely sort a `Path` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_path_slice_rev<P: AsRef<Path>>(slice: &mut [P]) {
    sort_slice_rev_unstable_by_path_key(slice, |e| e.as_ref())
}

// TODO -----------

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes_unstable(
    mut ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs.sort_unstable_by(|a, b| compare_str(a.1, b.1));

    ref_index_str_pairs_to_ref_indexes_inner(ref_index_str_pairs)
}

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes(
    mut ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs.sort_by(|a, b| compare_str(a.1, b.1));

    ref_index_str_pairs_to_ref_indexes_inner(ref_index_str_pairs)
}

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes_rev_unstable(
    mut ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs.sort_unstable_by(|a, b| compare_str(b.1, a.1));

    ref_index_str_pairs_to_ref_indexes_inner(ref_index_str_pairs)
}

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes_rev(
    mut ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs.sort_by(|a, b| compare_str(b.1, a.1));

    ref_index_str_pairs_to_ref_indexes_inner(ref_index_str_pairs)
}

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes_inner(
    ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs
        .into_iter()
        .enumerate()
        .filter_map(|(j, (i, _))| {
            if i != j {
                Some((i, j))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_ref_indexes<S>(slice: &mut [S], mut ref_indexes: Vec<(usize, usize)>) {
    let length = ref_indexes.len();

    for index in 0..length {
        let (i, j) = ref_indexes[index];

        slice.swap(i, j);

        for (t, _) in ref_indexes[index + 1..].iter_mut() {
            if *t == j {
                *t = i;
                break;
            }
        }
    }
}
