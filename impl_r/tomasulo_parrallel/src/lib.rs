pub mod simple_mutex{
    pub struct Mutex;
    impl Mutex {
        pub fn init() -> Mutex;
        pub fn lock(mutex: &mut Mutex) -> void;
        pub fn unlock(mutex: &mut Mutex) -> void;
    }
}
