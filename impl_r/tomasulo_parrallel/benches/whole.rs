#![feature(test)]

extern crate test;
use std::{hint::black_box, mem::MaybeUninit};

use test::Bencher;
use tomasulo_macro::tomasulo;
use tomasulo_parrallel::wrapper::Wrapper;

const MAIN_LOOP_ITERATION: u64 = 1000;
const SUB_LOOP_ITERATION: u64 = 1000;

pub fn user1(param: u64) -> u64 {
    let mut sum = 0;
    for i in black_box(0..param) {
        sum += black_box(i);
    }
    sum
}

pub fn user2(param: &u64) -> u64 {
    let mut sum = 0;
    for _ in black_box(0..SUB_LOOP_ITERATION) {
        sum += black_box(param);
    }
    sum
}

pub fn whole_programm() {
    for i in 0..MAIN_LOOP_ITERATION {
        let r = user1(i);
        user2(black_box(&r));
    }
}

#[bench]
pub fn whole_seq_bench(b: &mut Bencher) {
    b.iter(whole_programm);
}

pub fn whole_par() {
    let r = MaybeUninit::<Wrapper<u64,8>>::uninit();
}

pub fn whole_par_bench(b: &mut Bencher) {
    b.iter(whole_par);
}


#[tomasulo]
pub fn user1_tomasulo() -> u64 {
    let mut sum = 0;
    for i in black_box(0..SUB_LOOP_ITERATION) {
        sum += black_box(i);
    }
    sum
}

#[tomasulo]
pub fn user2_tomasulo(param: &u64) -> u64 {
    let mut sum = 0;
    for _ in black_box(0..SUB_LOOP_ITERATION) {
        sum += black_box(param);
    }
    sum
}

#[tomasulo]
pub fn whole_programm_tomasulo() {
    for i in 0..MAIN_LOOP_ITERATION {
        let r = user1(i);
        user2(black_box(&r));
    }
}
