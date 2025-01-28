#ifndef LOCKLESS_CHAIN_H
#define LOCKLESS_CHAIN_H
#include <stdatomic.h>
#include <sys/cdefs.h>
#include <stdbool.h>

typedef  struct chain_elt {
  struct chain_elt *next;
  char data[];
} chain_elt;

typedef struct {
  atomic_size_t head;
  atomic_size_t tail;
} chain;

//return next element in the chain, atomically, or NULL if chain is empty 
chain_elt* chain_pop(chain* ch);
//add next to the chain, atomically
void chain_push(chain *ch,chain_elt* next);

#endif
