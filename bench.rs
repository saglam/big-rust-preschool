#![feature(test)]

extern crate test;

use atoi::atoi;
use molecules;
use std::fs::File;
use std::io::{BufRead, BufReader};
use test::Bencher;

fn test_from_file(answer_file: &str, version: Version) {
    let question_file = &answer_file[..answer_file.len() - 2];
    let mut f = BufReader::new(File::open(question_file).unwrap());

    let mut buf = Vec::new();
    f.read_until(b' ', &mut buf).expect("n missing");
    let n: usize = atoi(&buf).unwrap();
    buf.clear();
    f.read_until(b' ', &mut buf).expect("l missing");
    let l: u32 = atoi(&buf).unwrap();
    buf.clear();
    f.read_until(b'\n', &mut buf).expect("u missing");
    let u: u32 = atoi(&buf).unwrap();
    buf.clear();

    let mut w: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        f.read_until(b' ', &mut buf).unwrap();
        w.push(atoi(&buf).unwrap());
        buf.clear();
    }

    let mut g = BufReader::new(File::open(answer_file).unwrap());
    g.read_until(b'\n', &mut buf).unwrap();
    buf.clear();
    g.read_until(b'\n', &mut buf).unwrap();
    let solvable = atoi::<u32>(&buf).unwrap() != 0;
    let answer = match version {
        Version::V1 => molecules::find_subset(l, u, &w),
        Version::V2 => molecules::find_subset2(l, u, &w),
    };
    molecules::assert_correct(&answer, l, u, &w, solvable);
}

enum Version {
    V1,
    V2,
}

#[bench]
fn small_tests_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 1..100 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V1);
        }
    });
}

#[bench]
fn medium_tests_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 100..110 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V1);
        }
    });
}

#[bench]
fn large_tests_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 110..120 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V1);
        }
    });
}

#[bench]
fn huge_tests_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 121..122 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V1);
        }
    });
}

#[bench]
fn small_tests_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 1..100 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V2);
        }
    });
}

#[bench]
fn medium_tests_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 100..110 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V2);
        }
    });
}

#[bench]
fn large_tests_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 110..120 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V2);
        }
    });
}

#[bench]
fn huge_tests_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 121..122 {
            test_from_file(&format!("testdata/{:02}.a", i), Version::V2);
        }
    });
}

