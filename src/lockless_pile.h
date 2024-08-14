#ifndef  LOCKLESS_CHAIN_H
#define LOCKLESS_CHAIN_H
#include <stdatomic.h>
#include "condition_wrap.h"

typedef struct {
  atomic_size_t head;
  condition_t waiter;
  size_t chain[];
} lockless_chain_t;

lockless_chain_t* chain_new(size_t max_size)
  __attribute_malloc__;
void chain_destroy(lockless_chain_t* chain);
void chain_add(lockless_chain_t* chain,size_t value);
size_t chain_get(lockless_chain_t* chain);
size_t chain_get_wait(lockless_chain_t* chain);

#endif
