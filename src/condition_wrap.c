#include "condition_wrap.h"


void cond_wait(condition_t * cond){
  pthread_mutex_lock(&cond->mut);
  while (! cond->flag) {
    pthread_cond_wait(&cond->cond, &cond->mut);
  }
  cond->flag=false;
  pthread_mutex_unlock(&cond->mut);
}

void cond_set(condition_t * cond){
  pthread_mutex_lock(&cond->mut);
  cond->flag = true;
  pthread_cond_signal(&cond->cond);
  pthread_mutex_unlock(&cond->mut);
}
void cond_unset(condition_t * cond){
  cond->flag = false;
};
bool condition_check(condition_t* cond){
  return cond->flag;
};
void condition_init(condition_t *cond){
  cond->flag = false;
  pthread_cond_init(&cond->cond, NULL);
  pthread_mutex_init(&cond->mut, NULL);
};
void condition_destroy(condition_t* cond){
  pthread_cond_destroy(&cond->cond);
  pthread_mutex_destroy(&cond->mut);
}
