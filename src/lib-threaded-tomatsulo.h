#include "condition_wrap.h"
#include "lockless_pile.h"
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>

void sample_fun(void *IOs_and_args[]);

size_t align4(size_t n);

typedef struct {
  atomic_char in_progress_inputs;
  typeof(sample_fun) *fun;
  void *IOs_and_args[];
} task;

enum {
  BUF_S_UNUSED,  //this is unused space, use it however you want
  BUF_S_IN_SCOPE,//used, may be used for futur tasks
  BUF_S_OUT_OF_SCOPE,//got out of scope, destroy whenever remaining uses reach 0
  BUF_S_WRAP,    //space up to end of pool was unused, bc next buffer wrapped around
} BUF_STATE;

typedef struct {
  volatile char state;
  atomic_size_t remaining_uses;
  size_t next_buf_addr; //actual size of whole struct, including data
  size_t data[];
} buffer;

typedef struct {
  //size_t poolsize2pwr;
  size_t poolsize;
  size_t head;
  size_t tail;
  size_t pool[];
} buf_pool;
