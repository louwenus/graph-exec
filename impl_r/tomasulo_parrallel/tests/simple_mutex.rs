use tomasulo_parrallel::simple_mutex::SimpleMutex;
use std::thread;

const NUM_THREAD: i8 = 100;
const NUM_ITERATION: i64 = 1000000;

static FUTEX: SimpleMutex = SimpleMutex::init();
static mut COUNTER: i64 = 0;

fn counter_thread() {
    let mut i = 0;
    while i < NUM_ITERATION {
        unsafe {
            SimpleMutex::lock(&FUTEX);
            COUNTER += 1;
            SimpleMutex::unlock(&FUTEX);
            i += 1;
        }
    }
}
#[test]
fn increment_protected() {
    let threads: Vec<_> = (0..NUM_THREAD)
        .map(|_| {
            thread::spawn(move || {
                counter_thread();
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }
    unsafe {
        assert_eq!(COUNTER, NUM_ITERATION * NUM_THREAD as i64);
    }
}
