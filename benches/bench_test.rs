#![feature(test)]

extern crate test;
use test::{Bencher, black_box};


use shuffler::{riffle, reverse, put_back, remove_middle};

#[cfg(feature = "random")]
use shuffler::random;


#[bench]
fn rufflering(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    b.iter(|| {
        riffle(&mut data);
    });
}

#[bench]
fn put_backering(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    b.iter(|| {
        put_back(&mut data, black_box(184));
    });
}


#[bench]
fn reversering(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    b.iter(|| {
        reverse(&mut data);
    });
}

#[bench]
fn remove_middleing(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    b.iter(|| {
        remove_middle(&mut data);
    });
}

#[bench]
fn combineing(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    b.iter(|| {
        riffle(&mut data);
        put_back(&mut data, 123);
        reverse(&mut data);
    
        riffle(&mut data);
        put_back(&mut data, 33);
        reverse(&mut data);
    
        riffle(&mut data);
        put_back(&mut data, 32);
        reverse(&mut data);
    
        riffle(&mut data);
        put_back(&mut data, 312);
        reverse(&mut data);
    });
}

#[cfg(feature = "random")]
#[bench]
fn randoming(b: &mut Bencher) {
    let mut data = Vec::from_iter(0..black_box(1000));
    let mut random_generator = rand::thread_rng();

    b.iter(|| {
        random(&mut data, &mut random_generator);
    });
}