//Small simple half-atomic queue.
//After initialisation, support concurently any number of pushing thread and a single poping thread
//ethier protect cross-read acess with a mutex, or read in only one thread 

#pragma once

#include <stdatomic.h>

typedef  struct chain_elt {
  struct chain_elt *next;
  char data[];
} chain_elt;

typedef struct {
  atomic_size_t head;
  atomic_size_t tail;
} chain;
//remove an element atomically respective to all other push, but no other pop
chain_elt* chain_pop(chain* ch);
//add an element atomically
void chain_push(chain *ch,chain_elt* next);
//initialize a chain
void chain_init(chain *ch);
