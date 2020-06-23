use std::cmp::Ord;
use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::SubAssign;

pub fn sumth_element<T, S>(a: &mut [T], mut sum: S) -> (usize, S)
where
    T: Ord,
    S: for<'a> Sum<&'a T> + SubAssign + Ord,
{
    let mut l = 0;
    let mut r = a.len();

    while l < r {
        let m = l + (r - l) / 2;
        order_stat::kth(&mut a[l..r], m - l);
        let tail_sum = a[l..=m].iter().sum();
        if tail_sum <= sum {
            l = m + 1;
            sum -= tail_sum;
        } else {
            r = m;
        }
    }
    (l, sum)
}

pub fn sumth_element_with<T, S, SumFn, CmpFn>(
    a: &mut [T],
    mut sum: S,
    sum_fn: SumFn,
    cmp_fn: CmpFn,
) -> (usize, S)
where
    S: SubAssign + Ord,
    SumFn: for<'a> Fn(&'a [T]) -> S,
    CmpFn: for<'a> Fn(&'a T, &'a T) -> Ordering,
{
    let mut l = 0;
    let mut r = a.len();

    while l < r {
        let m = l + (r - l) / 2;
        order_stat::kth_by(&mut a[l..r], m - l, |x, y| cmp_fn(x, y));
        let tail_sum = sum_fn(&a[l..=m]);
        if tail_sum <= sum {
            l = m + 1;
            sum -= tail_sum;
        } else {
            r = m;
        }
    }
    (l, sum)
}

#[test]
fn test_empty() {
    let mut a = [];
    assert_eq!(sumth_element(&mut a, 23), (0, 23));
}

#[test]
fn test_decreasing_0() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 0), (1, 0));
}

#[test]
fn test_decreasing_1() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 1), (2, 0));
}

#[test]
fn test_decreasing_2() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 3), (3, 0));
}

#[test]
fn test_decreasing_3() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 4), (3, 1));
}

#[test]
fn test_decreasing_4() {
    let mut a = [7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 28), (8, 0));
}

#[test]
fn test_stateful_1() {
    let mut a = [3, 2, 1, 0];
    assert_eq!(sumth_element(&mut a, 2), (2, 1));
    assert_eq!(sumth_element(&mut a, 2), (2, 1));
    assert_eq!(sumth_element(&mut a, 2), (2, 1));
    assert_eq!(sumth_element(&mut a, 3), (3, 0));
    assert_eq!(sumth_element(&mut a, 6), (4, 0));
    assert_eq!(sumth_element(&mut a, 7), (4, 1));
}

#[test]
fn test_stateful_2() {
    let mut a = [6, 7, 8, 2];
    assert_eq!(sumth_element(&mut a, 23), (4, 0));
    assert_eq!(sumth_element(&mut a, 1), (0, 1));
}
