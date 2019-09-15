//! # Alphanumeric Sort
//!
//! This crate can help you sort order for files and folders whose names contain numerals.
//!
//! ## Motives and Examples
//!
//! With the Rust native `sort` method, strings and paths are arranged into lexicographical order. It's natural, but in some cases, it is not so intuitive. For example, there are screen snap shots named by **shot-%N** like **shot-2**, **shot-1**, **shot-11**. After a lexicographical sorting, they will be ordered into **shot-1**, **shot-11**, **shot-2**. However, we would prefer **shot-1**, **shot-2**, **shot-11** mostly.
//!
//! ```rust
//! let mut names = ["shot-2", "shot-1", "shot-11"];
//!
//! names.sort();
//!
//! assert_eq!(["shot-1", "shot-11", "shot-2"], names);
//! ```
//!
//! Thus, in this kind of case, an alphanumeric sort might come in handy.
//!
//! ```rust
//! extern crate alphanumeric_sort;
//!
//! let mut names = ["shot-2", "shot-1", "shot-11"];
//!
//! alphanumeric_sort::sort_str_slice(&mut names);
//!
//! assert_eq!(["shot-1", "shot-2", "shot-11"], names);
//! ```
//!
//! ```rust
//! extern crate alphanumeric_sort;
//!
//! use std::path::Path;
//!
//! let mut paths = [Path::new("shot-2"), Path::new("shot-1"), Path::new("shot-11")];
//!
//! alphanumeric_sort::sort_path_slice(&mut paths);
//!
//! assert_eq!([Path::new("shot-1"), Path::new("shot-2"), Path::new("shot-11")], paths);
//! ```

use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::Path;

macro_rules! ordering_different_return {
    ($a:expr, $b:expr) => {{
        if $a > $b {
            return Ordering::Greater;
        } else if $a < $b {
            return Ordering::Less;
        }
    }};
}

macro_rules! ordering {
    ($a:expr, $b:expr) => {{
        if $a > $b {
            Ordering::Greater
        } else if $a < $b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }};
}

#[allow(clippy::while_let_on_iterator)]
/// Compare two strings.
pub fn compare_str<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> Ordering {
    let mut c1 = a.as_ref().chars();
    let mut c2 = b.as_ref().chars();

    let mut p1 = 0;
    let mut p2 = 0;

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
                            break;
                        }
                    }
                }
            }
        };
        p1 += 1;
        let cb = {
            match v2.take() {
                Some(c) => c,
                None => {
                    match c2.next() {
                        Some(c) => c,
                        None => {
                            break;
                        }
                    }
                }
            }
        };
        p2 += 1;

        if ca >= '0' && ca <= '9' && cb >= '0' && cb <= '9' {
            let mut da = f64::from(ca as u32) - f64::from(b'0');
            let mut db = f64::from(cb as u32) - f64::from(b'0');

            while let Some(ca) = c1.next() {
                if ca >= '0' && ca <= '9' {
                    da = da * 10.0 + (f64::from(ca as u32) - f64::from(b'0'));
                    p1 += 1;
                } else {
                    v1 = Some(ca);
                    break;
                }
            }

            while let Some(cb) = c2.next() {
                if cb >= '0' && cb <= '9' {
                    db = db * 10.0 + (f64::from(cb as u32) - f64::from(b'0'));
                    p2 += 1;
                } else {
                    v2 = Some(cb);
                    break;
                }
            }

            ordering_different_return!(da, db);
        } else if ca != cb {
            if (ca > (255 as char)) ^ (cb > (255 as char)) {
                return ordering!(cb, ca);
            } else {
                return ordering!(ca, cb);
            }
        }
    }

    return ordering!(p1, p2);
}

/// Compare two OsStr.
pub fn compare_os_str(a: &OsStr, b: &OsStr) -> Ordering {
    let sa = match a.to_str() {
        Some(s) => s,
        None => {
            return compare_os_str_inner(a, b);
        }
    };
    let sb = match b.to_str() {
        Some(s) => s,
        None => {
            return compare_os_str_inner(a, b);
        }
    };

    compare_str(sa, sb)
}

fn compare_os_str_inner(a: &OsStr, b: &OsStr) -> Ordering {
    a.partial_cmp(b).unwrap()
}

/// Sort a string slice.
pub fn sort_str_slice<S: AsRef<str>>(slice: &mut [S]) {
    slice.sort_by(|a, b| compare_str(a.as_ref(), b.as_ref()));
}

/// Sort a path slice.
pub fn sort_path_slice<P: AsRef<Path>>(slice: &mut [P]) {
    let mut use_str = true;

    let len = slice.len();

    {
        let mut paths_index = Vec::new();

        {
            let mut paths = Vec::with_capacity(len);

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

                paths_index.reserve(len);

                for (j, &(i, _)) in paths.iter().enumerate() {
                    if i != j {
                        paths_index.push((i, j))
                    }
                }
            }
        }

        if use_str {
            let len = paths_index.len();
            for index in 0..len {
                let (i, j) = paths_index[index];
                slice.swap(i, j);

                for (t, _) in paths_index.iter_mut().skip(index + 1) {
                    if *t == j {
                        *t = i;
                        break;
                    }
                }
            }
            return;
        }
    }

    sort_path_slice_inner(slice);
}

fn sort_path_slice_inner<P: AsRef<Path>>(slice: &mut [P]) {
    slice.sort_by(|a, b| compare_os_str_inner(a.as_ref().as_os_str(), b.as_ref().as_os_str()));
}
