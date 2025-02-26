#include "simple_mutex.h"

#include <stdatomic.h>
#include <uchar.h>
#include <linux/futex.h>
#include <sys/syscall.h>
#include <unistd.h>


//futex in this implementation have 3 states: 0: unlocked
//                                            1: locked, alone
//                                            >1: locked, someone await

void futex_lock(futex *mutex){
    char32_t tmp=0;
    if (atomic_compare_exchange_strong_explicit(mutex, &tmp,1,memory_order_consume,memory_order_relaxed)){
        return;
    }
    else {
        while (1) {
            tmp = atomic_exchange_explicit(mutex,2,memory_order_consume);
            if (tmp==0) {
                return;
            }
            syscall(SYS_futex, mutex, FUTEX_WAIT_PRIVATE, 2, NULL, NULL, 0);
        }
    }
}

void futex_unlock(futex *mutex){
    char32_t tmp;
    tmp = atomic_exchange_explicit(mutex, 0,memory_order_release);
    if (tmp!=1) {
        syscall(SYS_futex, mutex, FUTEX_WAKE_PRIVATE,1, NULL, NULL, 0);
    }
}

void futex_init(futex *mutex){
    atomic_store(mutex, 0);
}
