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

        if ca >= '0' && ca <= '9' && cb >= '0' && cb <= '9' {
            let mut da = f64::from(ca as u32) - f64::from(b'0');
            let mut db = f64::from(cb as u32) - f64::from(b'0');

            while let Some(ca) = c1.next() {
                if ca >= '0' && ca <= '9' {
                    da = da * 10.0 + (f64::from(ca as u32) - f64::from(b'0'));
                } else {
                    v1 = Some(ca);
                    break;
                }
            }

            while let Some(cb) = c2.next() {
                if cb >= '0' && cb <= '9' {
                    db = db * 10.0 + (f64::from(cb as u32) - f64::from(b'0'));
                } else {
                    v2 = Some(cb);
                    break;
                }
            }

            match da.partial_cmp(&db) {
                Some(ordering) if ordering != Ordering::Equal => {
                    return ordering;
                }
                _ => (),
            }
        } else {
            match ca.cmp(&cb) {
                Ordering::Equal => (),
                Ordering::Greater => {
                    return if (ca > (255 as char)) ^ (cb > (255 as char)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }
                Ordering::Less => {
                    return if (ca > (255 as char)) ^ (cb > (255 as char)) {
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
            return compare_os_str_inner(a, b);
        }
    };

    let sb = match b.as_ref().to_str() {
        Some(s) => s,
        None => {
            return compare_os_str_inner(a, b);
        }
    };

    compare_str(sa, sb)
}

#[cfg(feature = "std")]
#[inline]
fn compare_os_str_inner<A: AsRef<OsStr>, B: AsRef<OsStr>>(a: A, b: B) -> Ordering {
    a.as_ref().cmp(b.as_ref())
}

/// Compare two `CStr`.
#[cfg(feature = "std")]
#[inline]
pub fn compare_c_str<A: AsRef<CStr>, B: AsRef<CStr>>(a: A, b: B) -> Ordering {
    let sa = match a.as_ref().to_str() {
        Ok(s) => s,
        Err(_) => {
            return compare_c_str_inner(a, b);
        }
    };

    let sb = match b.as_ref().to_str() {
        Ok(s) => s,
        Err(_) => {
            return compare_c_str_inner(a, b);
        }
    };

    compare_str(sa, sb)
}

#[cfg(feature = "std")]
#[inline]
fn compare_c_str_inner<A: AsRef<CStr>, B: AsRef<CStr>>(a: A, b: B) -> Ordering {
    a.as_ref().cmp(b.as_ref())
}

/// Compare two `Path`.
#[cfg(feature = "std")]
#[inline]
pub fn compare_path<A: AsRef<Path>, B: AsRef<Path>>(a: A, b: B) -> Ordering {
    compare_os_str(a.as_ref(), b.as_ref())
}

// TODO -----------

/// Sort a slice by a `str` key.
#[inline]
pub fn sort_slice_by_str_key<A, T: ?Sized + AsRef<str>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_str(f(a), f(b)));
}

/// Sort a slice by a `OsStr` key.
#[cfg(feature = "std")]
pub fn sort_slice_by_os_str_key<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
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
        // fallback
        sort_slice_by_os_str_inner(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_os_str_inner<A, T: ?Sized + AsRef<OsStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_os_str_inner(f(a), f(b)));
}

/// Sort a slice by a `CStr` key.
#[cfg(feature = "std")]
pub fn sort_slice_by_c_str_key<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
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
        // fallback
        sort_slice_by_c_str_inner(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_c_str_inner<A, T: ?Sized + AsRef<CStr>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice.sort_by(|a, b| compare_c_str_inner(f(a), f(b)));
}

/// Sort a slice by a `Path` key.
#[cfg(feature = "std")]
pub fn sort_slice_by_path_key<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
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
        // fallback
        sort_slice_by_path_inner(slice, f);
    }
}

#[cfg(feature = "std")]
#[inline]
fn sort_slice_by_path_inner<A, T: ?Sized + AsRef<Path>, F: FnMut(&A) -> &T>(
    slice: &mut [A],
    mut f: F,
) {
    slice
        .sort_by(|a, b| compare_os_str_inner(f(a).as_ref().as_os_str(), f(b).as_ref().as_os_str()));
}

// TODO -----------

#[allow(clippy::redundant_closure)]
/// Sort a `str` slice.
#[inline]
pub fn sort_str_slice<S: AsRef<str>>(slice: &mut [S]) {
    slice.sort_by(|a, b| compare_str(a, b));
}

/// Sort an `OsStr` slice.
#[cfg(feature = "std")]
pub fn sort_os_str_slice<S: AsRef<OsStr>>(slice: &mut [S]) {
    sort_slice_by_os_str_key(slice, |e| e.as_ref())
}

/// Sort a `CStr` slice.
#[cfg(feature = "std")]
#[inline]
pub fn sort_c_str_slice<S: AsRef<CStr>>(slice: &mut [S]) {
    sort_slice_by_c_str_key(slice, |e| e.as_ref())
}

/// Sort a `Path` slice.
#[cfg(feature = "std")]
pub fn sort_path_slice<P: AsRef<Path>>(slice: &mut [P]) {
    sort_slice_by_path_key(slice, |e| e.as_ref())
}

// TODO -----------

#[cfg(feature = "std")]
#[inline]
fn ref_index_str_pairs_to_ref_indexes(
    mut ref_index_str_pairs: Vec<(usize, &str)>,
) -> Vec<(usize, usize)> {
    ref_index_str_pairs.sort_by(|a, b| compare_str(a.1, b.1));

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
