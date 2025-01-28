#include "lockless_pile_file.h"
#include <stdatomic.h>
#include <stddef.h>

chain_elt* chain_pop(chain *ch){
  chain_elt *tmp=(chain_elt*)ch->head;
  while (true) {
    if (tmp==0) {
      return 0;
    }
    chain_elt *next=tmp->next;
    if(atomic_compare_exchange_strong(&ch->head, (size_t*)&tmp, (size_t)next)){
      return tmp;
    }
  }
}

void chain_push(chain* ch, chain_elt *next){
  next->next=0;
  chain_elt** ptr=&next->next;
  ptr = (chain_elt**)atomic_exchange(&ch->tail, (size_t)ptr);
  *ptr=next;
}
