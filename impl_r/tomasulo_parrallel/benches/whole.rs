#![feature(test)]

extern crate test;
use std::hint::black_box;

use test::Bencher;
use tomasulo_macro::tomasulo;

const MAIN_LOOP_ITERATION: u64 = 10_000;
const SUB_LOOP_ITERATION: u64 = 10_000;

pub fn user1() -> u64 {
    let mut sum = 0;
    for i in black_box(0..SUB_LOOP_ITERATION) {
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
    for _ in 0..MAIN_LOOP_ITERATION {
        let r = user1();
        user2(black_box(&r));
    }
}

#[bench]
pub fn whole_programm_bench(b: &mut Bencher) {
    b.iter(whole_programm);
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
    for _ in 0..MAIN_LOOP_ITERATION {
        let r = user1();
        user2(black_box(&r));
    }
}
