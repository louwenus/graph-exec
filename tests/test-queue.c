#include <sched.h>
#include <stdatomic.h>
#include <pthread.h>
#include <stddef.h>
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>

#include "src/atomic_queue.h"

// Test structure with unique ID
typedef struct {
    chain_elt elt;
    int id;
} test_elt;

/***********************
 * Single-threaded Tests
 ***********************/

void test_init() {
    chain ch;
    chain_init(&ch);
    assert(atomic_load(&ch.head) == 0);
    assert(atomic_load(&ch.tail) == (size_t)&ch.head);
    printf("init_passed\n");
}

void test_single_push_pop() {
    chain ch;
    chain_init(&ch);
    test_elt elt;
    chain_push(&ch, (chain_elt*)&elt);
    
    assert(atomic_load(&ch.head) == (size_t)&elt);
    assert(atomic_load(&ch.tail) == (size_t)&elt);
    
    chain_elt* popped = chain_pop(&ch);
    assert(popped == (chain_elt*)&elt);
    assert(atomic_load(&ch.head) == 0);
    assert(atomic_load(&ch.tail) == (size_t)&ch.head);
    printf("single_passed\n");
}

void test_multiple_push_pop() {
    chain ch;
    chain_init(&ch);
    test_elt elt1, elt2, elt3;
    
    chain_push(&ch, (chain_elt*)&elt1);
    chain_push(&ch, (chain_elt*)&elt2);
    chain_push(&ch, (chain_elt*)&elt3);
    
    assert(chain_pop(&ch) == (chain_elt*)&elt1);
    assert(chain_pop(&ch) == (chain_elt*)&elt2);
    assert(chain_pop(&ch) == (chain_elt*)&elt3);
    assert(chain_pop(&ch) == NULL);
    printf("single_core_passed\n");
}

void test_pop_empty() {
    chain ch;
    chain_init(&ch);
    assert(chain_pop(&ch) == NULL);
    printf("empty_ok\n");
}

/***********************
 * Concurrent Push Test
 ***********************/
#define NUM_PUSH_THREADS 10
test_elt push_elements[NUM_PUSH_THREADS];
atomic_int push_cnt=0;

void* push_thread(void* arg) {
    int index = atomic_fetch_add(&push_cnt, 1);
    push_elements[index].id = index;
    chain_push((chain*)arg, (chain_elt*)&push_elements[index]);
    return NULL;
}

void test_concurrent_pushes() {
    chain ch;
    chain_init(&ch);
    pthread_t threads[NUM_PUSH_THREADS];
    
    for (int i = 0; i < NUM_PUSH_THREADS; i++) {
        pthread_create(&threads[i], NULL, push_thread, &ch);
    }
    
    for (int i = 0; i < NUM_PUSH_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    
    // Verify all elements were pushed
    bool found[NUM_PUSH_THREADS] = {false};
    chain_elt* popped;
    while ((popped = chain_pop(&ch)) != NULL) {
        test_elt* t_popped = (test_elt*)popped;
        assert(!found[t_popped->id]);
        found[t_popped->id] = true;
    }
    
    for (int i = 0; i < NUM_PUSH_THREADS; i++) {
        assert(found[i]);
    }
    printf("concurent_pushes_ok\n");
}

/***********************
 * Concurrent Pop Test
 ***********************/
#define NUM_POP_THREADS 10
test_elt pop_elements[NUM_POP_THREADS];
atomic_int popped_count = 0;
atomic_bool pop_found[NUM_POP_THREADS] = {false};

void* pop_thread(void* arg) {
    bool fa = 0;
    chain* ch = (chain*)arg;
    chain_elt* popped = chain_pop(ch);
    if (popped) {
        test_elt* t_popped = (test_elt*)popped;
        assert(atomic_compare_exchange_strong(&pop_found[t_popped->id],&fa,1));
        atomic_fetch_add(&popped_count, 1);
    }
    return NULL;
}

void test_concurrent_pops() {
    chain ch;
    chain_init(&ch);
    pthread_t threads[NUM_POP_THREADS];
    
    // Initialize elements and push them
    for (int i = 0; i < NUM_POP_THREADS; i++) {
        pop_elements[i].id = i;
        chain_push(&ch, (chain_elt*)&pop_elements[i]);
    }
    
    // Create pop threads
    for (int i = 0; i < NUM_POP_THREADS; i++) {
        pthread_create(&threads[i], NULL, pop_thread, &ch);
    }
    
    // Join threads
    for (int i = 0; i < NUM_POP_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    
    assert(popped_count == NUM_POP_THREADS);
    for (int i = 0; i < NUM_POP_THREADS; i++) {
        assert(pop_found[i]);
    }
    printf("concurent_pop_ok\n");
}

/***********************
 * Combined Concurrent Test
 ***********************/
#define NUM_CONCURRENT_THREADS 5
#define OPS_PER_THREAD 1000000
atomic_uint_fast64_t next_id = 0;
atomic_uint_fast64_t total_popped = 0;
atomic_bool* conc_popped;

void* concurrent_pusher(void* arg) {
    chain* ch = (chain*)arg;
    for (int i = 0; i < OPS_PER_THREAD; i++) {
        test_elt* elt = malloc(sizeof(test_elt));
        elt->id = atomic_fetch_add(&next_id, 1);
        chain_push(ch, (chain_elt*)elt);
    }
    return NULL;
}

void* concurrent_popper(void* arg) {
    chain* ch = (chain*)arg;
    bool fa=0;
    for (int i = 0; i < OPS_PER_THREAD*NUM_CONCURRENT_THREADS; i++) {
        chain_elt* popped;
        while (!(popped = chain_pop(ch))) {} // Keep trying until successful
        test_elt* t_popped = (test_elt*)popped;
        assert(t_popped->id < NUM_CONCURRENT_THREADS * OPS_PER_THREAD);
        assert(atomic_compare_exchange_strong(&conc_popped[t_popped->id], &fa, 1));
        total_popped++;
        free(popped);
    }
    return NULL;
}

void test_concurrent_ops() {
    chain ch;
    chain_init(&ch);
    conc_popped = calloc(NUM_CONCURRENT_THREADS * OPS_PER_THREAD, sizeof(bool));
    //pthread_t pushers[NUM_CONCURRENT_THREADS], poppers[NUM_CONCURRENT_THREADS];
    pthread_t pushers[NUM_CONCURRENT_THREADS], popper;
    
    // Create threads
    pthread_create(&popper, NULL, concurrent_popper, &ch);
    for (int i = 0; i < NUM_CONCURRENT_THREADS; i++) {
        pthread_create(&pushers[i], NULL, concurrent_pusher, &ch);
    }
    
    // Join threads
    for (int i = 0; i < NUM_CONCURRENT_THREADS; i++) {
        pthread_join(pushers[i], NULL);
    }
    pthread_join(popper, NULL);
    
    // Cleanup remaining elements
    //there should be none
    assert(chain_pop(&ch)==NULL);
    
    assert(total_popped == NUM_CONCURRENT_THREADS * OPS_PER_THREAD);
    free(conc_popped);
}

int main() {
    test_init();
    test_single_push_pop();
    test_multiple_push_pop();
    test_pop_empty();
    test_concurrent_pushes();
    //test_concurrent_pops(); Now declared INVALID ...
    test_concurrent_ops();
    
    printf("All tests passed!\n");
    return 0;
}
