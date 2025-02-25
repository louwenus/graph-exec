use std::{
    ptr::null_mut,
    sync::atomic::{
        AtomicPtr, AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
};
use tomasulo_parrallel::atomic_queue::{AtomicQueue, QueueElem};
use std::thread;

const NUM_THREAD: i8 = 5;
const NUM_ITERATION: i64 = 1000000;

static cnt : AtomicU64 = AtomicU64::new(0);



fn pusher(queue : &AtomicQueue<u64>) {
    let cur = 
}

#[test]
fn concurent_usage() -> () {
    let queue = AtomicQueue::<u64>::new();
    pusher(queue);
}
