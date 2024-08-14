#include "condition_wrap.h"
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>

void sample_fun(void *IOs_and_args[]);

typedef struct {
  atomic_char in_progress_inputs;
  typeof(sample_fun) *fun;
  void *IOs_and_args[];
} task;
