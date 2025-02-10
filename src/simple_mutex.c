#include "simple_mutex.h"

#include <uchar.h>
#include <linux/futex.h>
#include <sys/syscall.h>
#include <unistd.h>


//futex in this implementation have 3 states: 0: unlocked
//                                            1: locked, alone
//                                            >1: locked, someone await

void futex_lock(futex *mutex){
    char32_t tmp;
    tmp = atomic_fetch_add(mutex, 1);
    if (tmp==0) {
        return;
    }
    else {
        while (1) {
            syscall(SYS_futex, mutex, FUTEX_WAIT_PRIVATE, tmp+1, NULL, NULL, 0);
            tmp = atomic_fetch_add(mutex,2);
            if (tmp==0) {
                return;
            }
        }
    }
}

void futex_unlock(futex *mutex){
    char32_t tmp;
    tmp = atomic_fetch_sub(mutex, 1);
    if (tmp!=1) {
        atomic_store(mutex, 0);
        syscall(SYS_futex, mutex, FUTEX_WAKE_PRIVATE,1, NULL, NULL, 0);
    }
}

void futex_init(futex *mutex){
    atomic_store(mutex, 0);
}
