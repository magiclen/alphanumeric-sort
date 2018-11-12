extern crate alphanumeric_sort;

use std::cmp::Ordering;
use std::path::Path;

#[test]
fn compare_lv0_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("1", "2"));
}

#[test]
fn compare_lv0_2() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("1", "1"));
}

#[test]
fn compare_lv0_3() {
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("2", "1"));
}

#[test]
fn compare_lv1_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc", "bbb"));
}

#[test]
fn compare_lv1_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abcd", "bbb"));
}

#[test]
fn compare_lv1_3() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abcd", "bbbbb"));
}

#[test]
fn compare_lv1_4() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abcd", "abcd"));
}

#[test]
fn compare_lv2_1() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abc321", "abc321"));
}

#[test]
fn compare_lv2_2() {
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc3210", "abc321"));
}

#[test]
fn compare_lv3_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc320", "abc321"));
}

#[test]
fn compare_lv3_2() {
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc322", "abc321"));
}

#[test]
fn compare_lv4_1() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc1", "abc321"));
}

#[test]
fn compare_lv4_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc5", "abc321"));
}

#[test]
fn compare_lv4_3() {
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc567", "abc321"));
}

#[test]
fn compare_lv4_4() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc5d67", "abc321"));
}

#[test]
fn compare_lv5_1() {
    assert_eq!(Ordering::Equal, alphanumeric_sort::compare_str("abc123d123", "abc123d123"));
}

#[test]
fn compare_lv5_2() {
    assert_eq!(Ordering::Less, alphanumeric_sort::compare_str("abc123d1", "abc123d123"));
}

#[test]
fn compare_lv5_3() {
    assert_eq!(Ordering::Greater, alphanumeric_sort::compare_str("abc123d1234", "abc123d123"));
}

#[test]
fn sort_str_slice() {
    let mut array = ["第10-15-2章", "第1-2章", "第2-4章", "第2-33章", "第1章", "第1-4章", "第2-3章", "第1-11章", "第10-1章", "第3-1章", "第2-10章", "第2-2章", "第1-3章", "第10-15章", "第10-2章", "第10-15-1章", "第2-1章", "第2-12章", "第1-10章", "第3-10章"];

    alphanumeric_sort::sort_str_slice(&mut array);

    assert_eq!(["第1章", "第1-2章", "第1-3章", "第1-4章", "第1-10章", "第1-11章", "第2-1章", "第2-2章", "第2-3章", "第2-4章", "第2-10章", "第2-12章", "第2-33章", "第3-1章", "第3-10章", "第10-1章", "第10-2章", "第10-15章", "第10-15-1章", "第10-15-2章"], array);
}

#[test]
fn sort_path_slice() {
    let mut array = [Path::new("第10-15-2章"), Path::new("第1-2章"), Path::new("第2-4章"), Path::new("第2-33章"), Path::new("第1章"), Path::new("第1-4章"), Path::new("第2-3章"), Path::new("第1-11章"), Path::new("第10-1章"), Path::new("第3-1章"), Path::new("第2-10章"), Path::new("第2-2章"), Path::new("第1-3章"), Path::new("第10-15章"), Path::new("第10-2章"), Path::new("第10-15-1章"), Path::new("第2-1章"), Path::new("第2-12章"), Path::new("第1-10章"), Path::new("第3-10章")];

    alphanumeric_sort::sort_path_slice(&mut array);

    assert_eq!([Path::new("第1章"), Path::new("第1-2章"), Path::new("第1-3章"), Path::new("第1-4章"), Path::new("第1-10章"), Path::new("第1-11章"), Path::new("第2-1章"), Path::new("第2-2章"), Path::new("第2-3章"), Path::new("第2-4章"), Path::new("第2-10章"), Path::new("第2-12章"), Path::new("第2-33章"), Path::new("第3-1章"), Path::new("第3-10章"), Path::new("第10-1章"), Path::new("第10-2章"), Path::new("第10-15章"), Path::new("第10-15-1章"), Path::new("第10-15-2章")], array);
}