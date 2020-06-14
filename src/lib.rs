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

use std::path::Path;

let mut paths = [Path::new("shot-2"), Path::new("shot-1"), Path::new("shot-11")];

alphanumeric_sort::sort_path_slice(&mut paths);

assert_eq!([Path::new("shot-1"), Path::new("shot-2"), Path::new("shot-11")], paths);
```
*/

use std::cmp::Ordering;
use std::ffi::OsStr;
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

/// Compare two OsStr.
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

#[inline]
fn compare_os_str_inner<A: AsRef<OsStr>, B: AsRef<OsStr>>(a: A, b: B) -> Ordering {
    a.as_ref().cmp(b.as_ref())
}

/// Sort a `str` slice.
#[inline]
pub fn sort_str_slice<S: AsRef<str>>(slice: &mut [S]) {
    slice.sort_by(|a, b| compare_str(a, b));
}

/// Sort an `OsStr` slice.
#[inline]
pub fn sort_os_str_slice<S: AsRef<OsStr>>(slice: &mut [S]) {
    slice.sort_by(|a, b| compare_os_str(a, b));
}

/// Sort a path slice.
pub fn sort_path_slice<P: AsRef<Path>>(slice: &mut [P]) {
    let mut use_str = true;

    let length = slice.len();

    let mut paths = Vec::with_capacity(length);

    for (i, p) in slice.iter().enumerate() {
        let s = match p.as_ref().as_os_str().to_str() {
            Some(s) => s,
            None => {
                use_str = false;
                break;
            }
        };

        paths.push((i, s));
    }

    if use_str {
        paths.sort_by(|a, b| compare_str(a.1, b.1));

        let mut paths_index = Vec::with_capacity(length);

        for (j, &(i, _)) in paths.iter().enumerate() {
            if i != j {
                paths_index.push((i, j))
            }
        }

        let length = paths_index.len();

        for index in 0..length {
            let (i, j) = paths_index[index];

            slice.swap(i, j);

            for (t, _) in paths_index[index + 1..].iter_mut() {
                if *t == j {
                    *t = i;
                    break;
                }
            }
        }
    } else {
        // fallback
        sort_path_slice_inner(slice);
    }
}

#[inline]
fn sort_path_slice_inner<P: AsRef<Path>>(slice: &mut [P]) {
    slice.sort_by(|a, b| compare_os_str_inner(a.as_ref().as_os_str(), b.as_ref().as_os_str()));
}
