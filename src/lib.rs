//! # Alphanumeric Sort
//! This crate can help you sort order for files and folders whose names contain numerals.
//!
//! ## Motive and Examples
//!
//! With the Rust native `sort` method, strings and paths are arranged into lexicographical order. It's natural, but in some cases, it is not so intuitive. For example, there are screen snap shots named by **shot-%N** like **shot-2**, **shot-1**, **shot-11**. After a lexicographical sorting, they will be ordered into **shot-1**, **shot-11**, **shot-2**. However, we would prefer **shot-1**, **shot-2**, **shot-11** mostly.
//!
//! ```
//! let mut names = ["shot-2", "shot-1", "shot-11"];
//!
//! names.sort();
//!
//! assert_eq!(["shot-1", "shot-11", "shot-2"], names);
//! ```
//!
//! Thus, in this kind of case, an alphanumeric sort might come in handy.
//!
//! ```
//! extern crate alphanumeric_sort;
//!
//! let mut names = ["shot-2", "shot-1", "shot-11"];
//!
//! alphanumeric_sort::sort_str_slice(&mut names);
//!
//! assert_eq!(["shot-1", "shot-2", "shot-11"], names);
//! ```
//!
//! ```
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
use std::path::Path;
use std::ffi::OsStr;

macro_rules! ordering_different_return {
    ( $a:expr, $b:expr ) => {
        {
            if $a > $b {
                return Ordering::Greater;
            } else if $a < $b {
                return Ordering::Less;
            }
        }
    };
}

macro_rules! ordering {
    ( $a:expr, $b:expr ) => {
        {
            if $a > $b {
                Ordering::Greater
            } else if $a < $b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    };
}

/// Compare two strings.
pub fn compare_str(a: &str, b: &str) -> Ordering {
    let mut c1 = a.chars();
    let mut c2 = b.chars();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut v1: Option<char> = None;
    let mut v2: Option<char> = None;

    loop {
        let ca = {
            match v1.take() {
                Some(c) => c,
                None => match c1.next() {
                    Some(c) => c,
                    None => { break; }
                }
            }
        };
        p1 += 1;
        let cb = {
            match v2.take() {
                Some(c) => c,
                None => match c2.next() {
                    Some(c) => c,
                    None => { break; }
                }
            }
        };
        p2 += 1;

        if ca >= '0' && ca <= '9' && cb >= '0' && cb <= '9' {
            let mut da = ca as u32 - b'0' as u32;
            let mut db = cb as u32 - b'0' as u32;

            loop {
                let ca = match c1.next() {
                    Some(c) => c,
                    None => { break; }
                };

                if ca >= '0' && ca <= '9' {
                    da = da * 10 + (ca as u32 - b'0' as u32);
                    p1 += 1;
                } else {
                    v1 = Some(ca);
                    break;
                }
            }

            loop {
                let cb = match c2.next() {
                    Some(c) => c,
                    None => { break; }
                };

                if cb >= '0' && cb <= '9' {
                    db = db * 10 + (cb as u32 - b'0' as u32);
                    p2 += 1;
                } else {
                    v2 = Some(cb);
                    break;
                }
            }

            ordering_different_return!(da, db);
        } else {
            if ca != cb {
                if (ca > (255 as char)) ^ (cb > (255 as char)) {
                    return ordering!(cb, ca);
                } else {
                    return ordering!(ca, cb);
                }
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
pub fn sort_str_slice(slice: &mut [&str]) {
    slice.sort_by(|a, b| {
        compare_str(a, b)
    });
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
                paths.sort_by(|a, b| {
                    compare_str(a.1, b.1)
                });

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

                for index in (index + 1)..len {
                    let t = &mut paths_index[index];
                    if t.0 == j {
                        t.0 = i;
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
    let mut paths = Vec::new();

    for (i, p) in slice.iter().enumerate() {
        paths.push((i, i, p.as_ref().as_os_str()));
    }

    paths.sort_by(|a, b| {
        compare_os_str(a.2, b.2)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_lv0_1() {
        assert_eq!(Ordering::Less, compare_str("1", "2"));
    }

    #[test]
    fn test_compare_lv0_2() {
        assert_eq!(Ordering::Equal, compare_str("1", "1"));
    }

    #[test]
    fn test_compare_lv0_3() {
        assert_eq!(Ordering::Greater, compare_str("2", "1"));
    }

    #[test]
    fn test_compare_lv1_1() {
        assert_eq!(Ordering::Less, compare_str("abc", "bbb"));
    }

    #[test]
    fn test_compare_lv1_2() {
        assert_eq!(Ordering::Less, compare_str("abcd", "bbb"));
    }

    #[test]
    fn test_compare_lv1_3() {
        assert_eq!(Ordering::Less, compare_str("abcd", "bbbbb"));
    }

    #[test]
    fn test_compare_lv1_4() {
        assert_eq!(Ordering::Equal, compare_str("abcd", "abcd"));
    }

    #[test]
    fn test_compare_lv2_1() {
        assert_eq!(Ordering::Equal, compare_str("abc321", "abc321"));
    }

    #[test]
    fn test_compare_lv2_2() {
        assert_eq!(Ordering::Greater, compare_str("abc3210", "abc321"));
    }

    #[test]
    fn test_compare_lv3_1() {
        assert_eq!(Ordering::Less, compare_str("abc320", "abc321"));
    }

    #[test]
    fn test_compare_lv3_2() {
        assert_eq!(Ordering::Greater, compare_str("abc322", "abc321"));
    }

    #[test]
    fn test_compare_lv4_1() {
        assert_eq!(Ordering::Less, compare_str("abc1", "abc321"));
    }

    #[test]
    fn test_compare_lv4_2() {
        assert_eq!(Ordering::Less, compare_str("abc5", "abc321"));
    }

    #[test]
    fn test_compare_lv4_3() {
        assert_eq!(Ordering::Greater, compare_str("abc567", "abc321"));
    }

    #[test]
    fn test_compare_lv4_4() {
        assert_eq!(Ordering::Less, compare_str("abc5d67", "abc321"));
    }

    #[test]
    fn test_compare_lv5_1() {
        assert_eq!(Ordering::Equal, compare_str("abc123d123", "abc123d123"));
    }

    #[test]
    fn test_compare_lv5_2() {
        assert_eq!(Ordering::Less, compare_str("abc123d1", "abc123d123"));
    }

    #[test]
    fn test_compare_lv5_3() {
        assert_eq!(Ordering::Greater, compare_str("abc123d1234", "abc123d123"));
    }

    #[test]
    fn test_sort_str_slice() {
        let mut array = ["第10-15-2章", "第1-2章", "第2-4章", "第2-33章", "第1章", "第1-4章", "第2-3章", "第1-11章", "第10-1章", "第3-1章", "第2-10章", "第2-2章", "第1-3章", "第10-15章", "第10-2章", "第10-15-1章", "第2-1章", "第2-12章", "第1-10章", "第3-10章"];

        sort_str_slice(&mut array);

        assert_eq!(["第1章", "第1-2章", "第1-3章", "第1-4章", "第1-10章", "第1-11章", "第2-1章", "第2-2章", "第2-3章", "第2-4章", "第2-10章", "第2-12章", "第2-33章", "第3-1章", "第3-10章", "第10-1章", "第10-2章", "第10-15章", "第10-15-1章", "第10-15-2章"], array);
    }

    #[test]
    fn test_sort_path_slice() {
        let mut array = [Path::new("第10-15-2章"), Path::new("第1-2章"), Path::new("第2-4章"), Path::new("第2-33章"), Path::new("第1章"), Path::new("第1-4章"), Path::new("第2-3章"), Path::new("第1-11章"), Path::new("第10-1章"), Path::new("第3-1章"), Path::new("第2-10章"), Path::new("第2-2章"), Path::new("第1-3章"), Path::new("第10-15章"), Path::new("第10-2章"), Path::new("第10-15-1章"), Path::new("第2-1章"), Path::new("第2-12章"), Path::new("第1-10章"), Path::new("第3-10章")];

        sort_path_slice(&mut array);

        assert_eq!([Path::new("第1章"), Path::new("第1-2章"), Path::new("第1-3章"), Path::new("第1-4章"), Path::new("第1-10章"), Path::new("第1-11章"), Path::new("第2-1章"), Path::new("第2-2章"), Path::new("第2-3章"), Path::new("第2-4章"), Path::new("第2-10章"), Path::new("第2-12章"), Path::new("第2-33章"), Path::new("第3-1章"), Path::new("第3-10章"), Path::new("第10-1章"), Path::new("第10-2章"), Path::new("第10-15章"), Path::new("第10-15-1章"), Path::new("第10-15-2章")], array);
    }
}
