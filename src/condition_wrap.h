#ifndef CONDITIONS_WRAP_H
#define CONDITIONS_WRAP_H

#include <pthread.h>
#include <stdatomic.h>
#include <stdbool.h>

typedef struct {
  pthread_cond_t cond;
  pthread_mutex_t mut;
  volatile bool flag;
} condition_t ;

void cond_wait(condition_t * cond);
void cond_set(condition_t * cond);
void cond_unset(condition_t * cond);
bool condition_check(condition_t* cond);
void condition_init(condition_t* cond);
void condition_destroy(condition_t* cond);

#endif // CONDITIONS_WRAP_H
