#include "src/atomic_queue.h"
#include <stdatomic.h>
#include <stdint.h>
#include <pthread.h>
#include <stdlib.h>

typedef struct {
    chain_elt elt;
    int64_t id;
} test_elt;


#define NUM_CONCURRENT_THREADS 1LL
#define OPS_PER_THREAD 5000000LL

atomic_uint_fast64_t next_id = 0;

typedef struct {
    chain *ch;
    test_elt *elts;
} env;

void* pusher(void* arg){
    env e = *(env*)arg;
    for (int64_t i=0; i<OPS_PER_THREAD; i++) {
        int64_t next=atomic_fetch_add_explicit(&next_id, 1, memory_order_relaxed);
        chain_push(e.ch, (chain_elt*)(e.elts+next));
    }
    return NULL;
}

void* collector(void* arg){
    chain* ch=arg;
    for (int64_t i=0; i<OPS_PER_THREAD*NUM_CONCURRENT_THREADS; i++) {
        test_elt* next;
        while (!(next = (test_elt*)chain_pop(ch))){
            ;
        }
        if (next->id==-1) {
            exit(1);
        }
        next->id=-1;
    }
    return NULL;
}

int main(){
    chain ch;
    test_elt *elems=malloc(NUM_CONCURRENT_THREADS*OPS_PER_THREAD*sizeof(test_elt));
    chain_init(&ch);
    for (int64_t i=0; i<NUM_CONCURRENT_THREADS*OPS_PER_THREAD; i++) {
        elems[i].id=i;
    }
    env e={&ch,elems};
    pthread_t thp[NUM_CONCURRENT_THREADS];
    pthread_t coll;
    void* garbage;
    pthread_create(&coll, NULL, collector, &ch);
    for (int i=0; i<NUM_CONCURRENT_THREADS; i++) {
        pthread_create(&thp[i], NULL, pusher, &e);    
    }
    pthread_join(coll, &garbage);
    for (int i=0; i<NUM_CONCURRENT_THREADS; i++) {
        pthread_join(thp[i], &garbage);
    }
    for (int64_t i=0; i<NUM_CONCURRENT_THREADS*OPS_PER_THREAD; i++) {
        if (elems[i].id!=-1) {
            exit(1);
        }
    }
    return 0;
}
