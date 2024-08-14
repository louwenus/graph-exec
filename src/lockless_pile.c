#include "lockless_pile.h"
#include "condition_wrap.h"
#include <stdatomic.h>
#include <stddef.h>
#include <stdlib.h>

lockless_chain_t *chain_new(size_t max_size) {
  lockless_chain_t *new =
      malloc(sizeof(lockless_chain_t) + sizeof(void *) * (max_size + 1));
  if (new == NULL) {
    exit(EXIT_FAILURE);
  }
  new->head = 0;
  condition_init(&new->waiter);
  return new;
}
void chain_destroy(lockless_chain_t *chain) {
  condition_destroy(&chain->waiter);
  free(chain);
}

void chain_add(lockless_chain_t *chain, size_t value) {
  size_t previous_head = 0;
  chain->chain[value] = 0;
  while (atomic_compare_exchange_weak(&chain->head, &previous_head, value)) {
    chain->chain[value] = previous_head;
  }
  if (previous_head == 0) {
    cond_set(&chain->waiter);
  }
}

size_t chain_get(lockless_chain_t *chain) {
  size_t new_head = 0;
  size_t ret = 0;
  while (atomic_compare_exchange_weak(&chain->head, &ret, new_head)) {
    new_head = chain->chain[ret];
  }
  return ret;
}
size_t chain_get_wait(lockless_chain_t *chain) {
  size_t ret = chain_get(chain);
  if (ret == 0) {
    while (ret == 0) {
      cond_wait(&chain->waiter);
      ret = chain_get(chain);
    }
    cond_set(&chain->waiter);
  }
  return ret;
}
