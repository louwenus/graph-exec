use atomic::{Atomic, Ordering};

pub struct Mutex {
    state:Atomic<i8>
}
impl Mutex {
    pub fn init() -> Mutex {
        Mutex {
            state:0
        }
    }
    pub fn lock(mutex: &mut Mutex) -> void {
        tmp:i8=0;
        
    }
}
