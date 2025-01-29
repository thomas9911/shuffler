use std::collections::HashMap;

#[cfg(feature = "random")]
use rand::Rng;

use std::hash::{Hasher, BuildHasher};

struct IntHasher(u64);

impl Hasher for IntHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("invalid use")
    }

    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64
    }
}

impl BuildHasher for IntHasher{
    type Hasher = IntHasher;

    fn build_hasher(&self) -> Self::Hasher {
        IntHasher(0)
    }
}

pub fn reverse<T>(data: &mut [T]) {
    data.reverse();
}

pub fn put_back<T>(data: &mut [T], amount: usize) {
    data.rotate_left(amount);
}

pub fn riffle<T>(data: &mut [T]) {
    #[inline]
    fn get_or_return(lookup: &HashMap<usize, usize, IntHasher>, index: usize) -> usize {
        *(lookup.get(&index).unwrap_or(&index))
    }

    let middle = (data.len() as f32 / 2.0).round() as usize;

    let mut i = 0;
    let mut j = 0;
    let mut lookup = HashMap::<usize, usize, IntHasher>::with_hasher(IntHasher(0));

    while i < data.len() {
        let found = get_or_return(&lookup, j);
        if found >= data.len() {
            return;
        }
        data.swap(i, found);
        lookup.insert(i, found);

        let found = get_or_return(&lookup, middle + j);
        if found >= data.len() {
            return;
        }
        data.swap(i + 1, found);
        lookup.insert(i + 1, found);

        i += 2;
        j += 1;
    }
}

pub fn remove_middle<T>(data: &mut [T]) {
    let quarter_length = data.len() / 4;
    let remainder = data.len() - (quarter_length * 4);

    let (left, right) = data.split_at_mut(quarter_length*2 + remainder);
    let (_left_left, left_right) = left.split_at_mut(quarter_length + remainder);
    let (right_left, right_right) = right.split_at_mut(quarter_length);

    left_right.swap_with_slice(right_left);
    right_right.swap_with_slice(left_right);
}

#[cfg(feature = "random")]
pub fn random<T, R: Rng>(data: &mut [T], random_generator: &mut R) {
    for i in 0..data.len() {
        let n: usize = random_generator.gen_range(0..data.len());
        data.swap(i, n);
    }
}

#[test]
fn reverse_test() {
    let mut v = vec![1, 2, 3];
    reverse(&mut v);
    assert_eq!(v, vec![3, 2, 1]);
}

#[test]
fn put_back_test() {
    let mut v = vec![1, 2, 3, 4];
    put_back(&mut v, 2);
    assert_eq!(v, vec![3, 4, 1, 2]);
}

#[test]
fn riffle_test_even() {
    let mut r = Vec::from_iter(1..=10);
    riffle(&mut r);
    assert_eq!(r, vec![1, 6, 2, 7, 3, 5, 4, 9, 8, 10]);
}

#[test]
fn riffle_test_odd() {
    let mut r = Vec::from_iter(1..=9);
    riffle(&mut r);
    assert_eq!(r, vec![1, 6, 2, 7, 3, 5, 4, 9, 8]);
}

#[test]
fn remove_middle_test() {
    let mut r = Vec::from_iter(1..=9);
    remove_middle(&mut r);
    assert_eq!(r, vec![1, 2, 3, 8, 9, 4, 5, 6, 7]);
}

#[test]
fn combine_test() {
    let mut r = Vec::from_iter(1..=15);

    riffle(&mut r);
    put_back(&mut r, 3);
    reverse(&mut r);

    assert_eq!(r, vec![2, 9, 1, 14, 15, 13, 8, 6, 7, 5, 12, 4, 11, 3, 10]);

    riffle(&mut r);
    put_back(&mut r, 3);
    reverse(&mut r);

    assert_eq!(r, vec![9, 7, 2, 3, 10, 11, 6, 13, 8, 15, 4, 14, 12, 1, 5]);

    riffle(&mut r);
    put_back(&mut r, 3);
    reverse(&mut r);

    assert_eq!(r, vec![7, 8, 9, 1, 5, 12, 13, 11, 6, 10, 14, 3, 4, 2, 15]);

    riffle(&mut r);
    put_back(&mut r, 3);
    reverse(&mut r);

    assert_eq!(r, vec![8, 6, 7, 2, 15, 4, 11, 12, 13, 5, 3, 1, 14, 9, 10]);
}

#[cfg(all(feature = "random", test))]
use rand::rngs::mock::StepRng;

#[test]
#[cfg(feature = "random")]
fn test_random() {
    let mut rng = StepRng::new(0, 1);
    let mut r = Vec::from_iter(1..=10);
    let expected = vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    random(&mut r, &mut rng);
    assert_eq!(r, expected);

    let mut rng = rand::thread_rng();
    let mut r = Vec::from_iter(1..=10);
    random(&mut r, &mut rng);
    assert_ne!(r, Vec::from_iter(1..=10));
    assert_ne!(r, expected);
}
