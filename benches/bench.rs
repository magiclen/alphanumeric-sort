extern crate alphanumeric_sort;

#[macro_use]
extern crate bencher;

use std::path::Path;

use bencher::Bencher;

fn sort_path_slice(bencher: &mut Bencher) {
    bencher.iter(|| {
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

        array
    });
}

fn compare_path(bencher: &mut Bencher) {
    bencher.iter(|| {
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

        array.sort_by(|a, b| alphanumeric_sort::compare_path(a, b));

        array
    });
}

benchmark_group!(sort_path_compare_path, sort_path_slice, compare_path);
benchmark_main!(sort_path_compare_path);
