#include "atomic_queue.h"
#include <stdatomic.h>
#include <stddef.h>

chain_elt *chain_pop(chain *ch) {
  chain_elt *tmp = (chain_elt *)ch->head;
  while (true) {
    if (tmp == 0) {
      return 0;
    }
    chain_elt *next = tmp->next;
    if (next == 0) {
      if (atomic_compare_exchange_strong(&ch->tail, (size_t *)&tmp,
                                         (size_t)&ch->head)) {
        chain_elt *tmpcpy = tmp;
        atomic_compare_exchange_strong(&ch->head, (size_t *)&tmp, 0);
        return tmpcpy;
      } else {
        tmp = (chain_elt *)ch->head;
      }
    } else {
      if (atomic_compare_exchange_strong(&ch->head, (size_t *)&tmp,
                                         (size_t)next)) {
        return tmp;
      }
    }
  }
}

void chain_push(chain *ch, chain_elt *next) {
  next->next = 0;
  chain_elt **ptr = &next->next;
  ptr = (chain_elt **)atomic_exchange(&ch->tail, (size_t)ptr);
  *ptr = next;
}

void chain_init(chain *ch) {
  ch->head = 0;
  ch->tail = (atomic_size_t)&ch->head;
}
