#include "atomic_queue.h"

#include <sched.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stddef.h>

void y() { sched_yield(); }

// due to a small, hard-to-fix race condition with concurent pop if an element
// is half-poped, then concurently poped and pushed again, this function is
// designed to be atomic only respectively to another PUSH, not another POP
// ethier POP in a single thread or protect POP with a simple mutex
chain_elt *chain_pop(chain *ch) {
    chain_elt *tmp = (chain_elt *)atomic_load_explicit(&ch->head,memory_order_consume);
    if (tmp == 0) {
        return 0;
    }
    y();
    volatile chain_elt *next = tmp->next;
    while (next == 0) {
        chain_elt *tmpcpy = tmp;
        if (atomic_compare_exchange_strong_explicit(&ch->tail, (size_t *)&tmpcpy,
                                           (size_t)&ch->head,memory_order_relaxed,memory_order_relaxed)) {
            // atomic here in case already changed by a write
            y();
            atomic_compare_exchange_strong_explicit(&ch->head, (size_t *)&tmpcpy, 0, memory_order_relaxed, memory_order_relaxed);
            return tmp;
        } // retry, write is in progress
          // maybe we could yield ?
        next = tmp->next;
    }
    atomic_store_explicit(&ch->head, (size_t)next, memory_order_relaxed);
    return tmp;
}

void chain_push(chain *ch, chain_elt *next) {
    y();
    next->next = 0;
    y();
    chain_elt **ptr = &next->next;
    y();
    ptr = (chain_elt **)atomic_exchange_explicit(&ch->tail, (size_t)ptr,
                                                 memory_order_release);
    y();
    *ptr = next;
}

void chain_init(chain *ch) {
    ch->head = 0;
    ch->tail = (atomic_size_t)&ch->head;
}
