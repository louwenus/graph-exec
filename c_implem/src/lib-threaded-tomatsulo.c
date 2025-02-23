#include "lib-threaded-tomatsulo.h"
#include <sched.h>


/*
 Code -> MAIN PROCESS
          |       |
        create  attribute, mark out of scope
          |       |
        TASKS - IO BUFFERS     <-- manage-, destroy
          |
        when all bufers ready             |
          |
        READY PILE  --read by-> WORKERS >_/

*/

static const size_t buffer_head_siz = (sizeof(buffer)+3)/sizeof(size_t); //size of buffer metadata with unit in size_t

size_t align4(size_t n){
  return  (n+3) & (~ 3ULL);
}

//warning: return NULL if cannot allocate due to lack of
buffer* next_buff_addr(buf_pool* pool,size_t desired_space)
{
  size_t next=((buffer*)(pool->pool+pool->head))->next_buf_addr;
  if (pool->head > pool->tail) {
    
  }
  
}
