#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdatomic.h>
#include "condition_wrap.h"

void sample_function(void *context,void *accumulator);
typedef typeof(sample_function) task_proto_t;

typedef struct task {
  task_proto_t * const task_func;
  bool const has_accumulator;
  atomic_ptrdiff_t last_executed;
  
  
  char accumulator[];
} task_t ;

typedef struct {
  size_t const size;
  task_t *tasks;
} taskgroup ;

typedef struct{
  size_t cur_task;
  
} thread_enviro ;
