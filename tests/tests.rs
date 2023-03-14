use std::cmp::Ordering;

#[cfg(feature = "std")]
use std::path::Path;

#[test]
fn compare_lv0_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("0", "1"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("1", "0"));
}

#[test]
fn compare_lv0_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "2"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("2", "1"));
}

#[test]
fn compare_lv0_3() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("0", "0"));
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("1", "1"));
}

#[test]
fn compare_lv1_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc", "bbb"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("bbb", "abc"));
}

#[test]
fn compare_lv1_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abcd", "bbb"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("bbb", "abcd"));
}

#[test]
fn compare_lv1_3() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abcd", "bbbbb"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("bbbbb", "abcd"));
}

#[test]
fn compare_lv1_5() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abcd", "abcd"));
}

#[test]
fn compare_lv2_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc321", "abc3210"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc3210", "abc321"));
}

#[test]
fn compare_lv2_2() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abc321", "abc321"));
}

#[test]
fn compare_lv3_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc320", "abc321"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc321", "abc320"));
}

#[test]
fn compare_lv4_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc1", "abc321"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc321", "abc1"));
}

#[test]
fn compare_lv4_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc5", "abc321"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc321", "abc5"));
}

#[test]
fn compare_lv4_3() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc321", "abc567"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc567", "abc321"));
}

#[test]
fn compare_lv4_4() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc5d67", "abc321"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc321", "abc5d67"));
}

#[test]
fn compare_lv5_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc123d1", "abc123d123"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc123d123", "abc123d1"));
}

#[test]
fn compare_lv5_2() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abc123d123", "abc123d123"));
}

#[test]
fn compare_lv6_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "1a"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("1a", "1"));
}

#[test]
fn compare_lv7_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "中"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("中", "1"));
}

#[test]
fn compare_lv8_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1個", "1.1個"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("1.1個", "1個"));
}

#[test]
fn compare_lv9_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "01"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("01", "1"));
}

#[test]
fn compare_lv9_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1章", "01章"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("01章", "1章"));
}

#[test]
fn compare_lv9_3() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "10"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("10", "1"));
}

#[test]
fn compare_lv9_4() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1章", "10章"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("10章", "1章"));
}

#[test]
fn compare_lv10_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("00001", "000001"));
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("000001", "00001"));
}

#[test]
fn compare_lv11_1() {
    assert_eq!(alphanumeric_sort::compare_str(
        "23478435345672365487236435437465873645736452658734658734653645872542736437465365487326548734658736457265345736458735",
        "23478435345672365487236435437465873645736452658734658734653645872542736437465365487326548734658736457265345736458736"
    ), Ordering::Less);

    assert_eq!(alphanumeric_sort::compare_str(
        "23478435345672365487236435437465873645736452658734658734653645872542736437465365487326548734658736457265345736458736",
        "23478435345672365487236435437465873645736452658734658734653645872542736437465365487326548734658736457265345736458735",
    ), Ordering::Greater);
}

#[test]
fn sort_str_slice() {
    let mut array = [
        "第10-15-2章",
        "第1-2章",
        "第2-4章",
        "第2-33章",
        "第1章",
        "第1-4章",
        "第2-3章",
        "第1-11章",
        "第10-1章",
        "第3-1章",
        "第2-10章",
        "第2-2章",
        "第1-3章",
        "第10-15章",
        "第10-2章",
        "第10-15-1章",
        "第2-1章",
        "第2-12章",
        "第1-10章",
        "第3-10章",
    ];

    alphanumeric_sort::sort_str_slice(&mut array);

    assert_eq!(
        [
            "第1章",
            "第1-2章",
            "第1-3章",
            "第1-4章",
            "第1-10章",
            "第1-11章",
            "第2-1章",
            "第2-2章",
            "第2-3章",
            "第2-4章",
            "第2-10章",
            "第2-12章",
            "第2-33章",
            "第3-1章",
            "第3-10章",
            "第10-1章",
            "第10-2章",
            "第10-15章",
            "第10-15-1章",
            "第10-15-2章"
        ],
        array
    );
}

#[test]
fn sort_slice_by_str_key() {
    #[derive(Debug)]
    struct A(&'static str);

    impl From<&'static str> for A {
        #[inline]
        fn from(s: &'static str) -> Self {
            A(s)
        }
    }

    impl PartialEq<A> for &str {
        #[inline]
        fn eq(&self, other: &A) -> bool {
            self == &other.0
        }
    }

    let mut array = [
        A::from("第10-15-2章"),
        A::from("第1-2章"),
        A::from("第2-4章"),
        A::from("第2-33章"),
        A::from("第1章"),
        A::from("第1-4章"),
        A::from("第2-3章"),
        A::from("第1-11章"),
        A::from("第10-1章"),
        A::from("第3-1章"),
        A::from("第2-10章"),
        A::from("第2-2章"),
        A::from("第1-3章"),
        A::from("第10-15章"),
        A::from("第10-2章"),
        A::from("第10-15-1章"),
        A::from("第2-1章"),
        A::from("第2-12章"),
        A::from("第1-10章"),
        A::from("第3-10章"),
    ];

    alphanumeric_sort::sort_slice_by_str_key(&mut array, |e| e.0);

    assert_eq!(
        [
            "第1章",
            "第1-2章",
            "第1-3章",
            "第1-4章",
            "第1-10章",
            "第1-11章",
            "第2-1章",
            "第2-2章",
            "第2-3章",
            "第2-4章",
            "第2-10章",
            "第2-12章",
            "第2-33章",
            "第3-1章",
            "第3-10章",
            "第10-1章",
            "第10-2章",
            "第10-15章",
            "第10-15-1章",
            "第10-15-2章"
        ],
        array
    );
}

#[cfg(feature = "std")]
#[test]
fn sort_path_slice() {
    let mut array = [
        Path::new("第10-15-2章"),
        Path::new("第1-2章"),
        Path::new("第2-4章"),
        Path::new("第2-33章"),
        Path::new("第1章"),
        Path::new("第1-4章"),
        Path::new("第2-3章"),
        Path::new("第1-11章"),
        Path::new("第10-1章"),
        Path::new("第3-1章"),
        Path::new("第2-10章"),
        Path::new("第2-2章"),
        Path::new("第1-3章"),
        Path::new("第10-15章"),
        Path::new("第10-2章"),
        Path::new("第10-15-1章"),
        Path::new("第2-1章"),
        Path::new("第2-12章"),
        Path::new("第1-10章"),
        Path::new("第3-10章"),
    ];

    alphanumeric_sort::sort_path_slice(&mut array);

    assert_eq!(
        [
            Path::new("第1章"),
            Path::new("第1-2章"),
            Path::new("第1-3章"),
            Path::new("第1-4章"),
            Path::new("第1-10章"),
            Path::new("第1-11章"),
            Path::new("第2-1章"),
            Path::new("第2-2章"),
            Path::new("第2-3章"),
            Path::new("第2-4章"),
            Path::new("第2-10章"),
            Path::new("第2-12章"),
            Path::new("第2-33章"),
            Path::new("第3-1章"),
            Path::new("第3-10章"),
            Path::new("第10-1章"),
            Path::new("第10-2章"),
            Path::new("第10-15章"),
            Path::new("第10-15-1章"),
            Path::new("第10-15-2章")
        ],
        array
    );
}

#[cfg(feature = "std")]
#[test]
fn sort_slice_by_path_key() {
    #[derive(Debug)]
    struct A(&'static Path);

    impl From<&'static str> for A {
        #[inline]
        fn from(s: &'static str) -> Self {
            A(Path::new(s))
        }
    }

    impl PartialEq<A> for &Path {
        #[inline]
        fn eq(&self, other: &A) -> bool {
            self == &other.0
        }
    }

    let mut array = [
        A::from("第10-15-2章"),
        A::from("第1-2章"),
        A::from("第2-4章"),
        A::from("第2-33章"),
        A::from("第1章"),
        A::from("第1-4章"),
        A::from("第2-3章"),
        A::from("第1-11章"),
        A::from("第10-1章"),
        A::from("第3-1章"),
        A::from("第2-10章"),
        A::from("第2-2章"),
        A::from("第1-3章"),
        A::from("第10-15章"),
        A::from("第10-2章"),
        A::from("第10-15-1章"),
        A::from("第2-1章"),
        A::from("第2-12章"),
        A::from("第1-10章"),
        A::from("第3-10章"),
    ];

    alphanumeric_sort::sort_slice_by_path_key(&mut array, |e| e.0);

    assert_eq!(
        [
            Path::new("第1章"),
            Path::new("第1-2章"),
            Path::new("第1-3章"),
            Path::new("第1-4章"),
            Path::new("第1-10章"),
            Path::new("第1-11章"),
            Path::new("第2-1章"),
            Path::new("第2-2章"),
            Path::new("第2-3章"),
            Path::new("第2-4章"),
            Path::new("第2-10章"),
            Path::new("第2-12章"),
            Path::new("第2-33章"),
            Path::new("第3-1章"),
            Path::new("第3-10章"),
            Path::new("第10-1章"),
            Path::new("第10-2章"),
            Path::new("第10-15章"),
            Path::new("第10-15-1章"),
            Path::new("第10-15-2章")
        ],
        array
    );
}
