#include <stdatomic.h>
#include <pthread.h>
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>

#include "src/simple_mutex.h"
/***********************
 * Basic Functionality Tests
 ***********************/

void test_futex_init() {
    futex m;
    futex_init(&m);
    
    // Test initial state by trying to immediately lock/unlock
    futex_lock(&m);
    futex_unlock(&m);
}

void test_lock_unlock_sequence() {
    futex m;
    futex_init(&m);
    
    for (int i = 0; i < 10; i++) {
        futex_lock(&m);
        futex_unlock(&m);
    }
}

/***********************
 * Concurrent Access Tests
 ***********************/
#define NUM_THREADS 10
#define NUM_ITERATIONS 1000

volatile int shared_counter = 0;
futex counter_mutex;

void* counter_thread(void* arg) {
    for (int i = 0; i < NUM_ITERATIONS; i++) {
        futex_lock(&counter_mutex);
        shared_counter++;
        futex_unlock(&counter_mutex);
    }
    return NULL;
}

void test_concurrent_access() {
    shared_counter = 0;
    futex_init(&counter_mutex);
    pthread_t threads[NUM_THREADS];
    
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, counter_thread, NULL);
    }
    
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    
    assert(shared_counter == NUM_THREADS * NUM_ITERATIONS);
}

/***********************
 * Mutual Exclusion Test
 ***********************/
futex mutex;
pthread_mutex_t test_mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t test_cond = PTHREAD_COND_INITIALIZER;
bool thread1_has_lock = false;

void* thread1_func(void* arg) {
    futex_lock(&mutex);
    
    // Signal that we have the lock
    pthread_mutex_lock(&test_mutex);
    thread1_has_lock = true;
    pthread_cond_signal(&test_cond);
    pthread_mutex_unlock(&test_mutex);
    
    // Hold lock for a while
    sleep(1);
    futex_unlock(&mutex);
    return NULL;
}

void* thread2_func(void* arg) {
    // Wait for thread1 to get the lock
    pthread_mutex_lock(&test_mutex);
    while (!thread1_has_lock) {
        pthread_cond_wait(&test_cond, &test_mutex);
    }
    pthread_mutex_unlock(&test_mutex);
    
    // Try to acquire the lock
    futex_lock(&mutex);
    // If we get here, thread1 must have released the lock
    futex_unlock(&mutex);
    return NULL;
}

void test_mutual_exclusion() {
    futex_init(&mutex);
    thread1_has_lock = false;
    
    pthread_t t1, t2;
    pthread_create(&t1, NULL, thread1_func, NULL);
    pthread_create(&t2, NULL, thread2_func, NULL);
    
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
}

/***********************
 * Stress Test
 ***********************/
#define STRESS_THREADS 20
#define STRESS_ITERATIONS 500

void* stress_thread(void* arg) {
    futex* m = (futex*)arg;
    for (int i = 0; i < STRESS_ITERATIONS; i++) {
        futex_lock(m);
        usleep(100); // Add some delay to increase contention
        futex_unlock(m);
    }
    return NULL;
}

void test_stress() {
    futex m;
    futex_init(&m);
    pthread_t threads[STRESS_THREADS];
    
    for (int i = 0; i < STRESS_THREADS; i++) {
        pthread_create(&threads[i], NULL, stress_thread, &m);
    }
    
    for (int i = 0; i < STRESS_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    
    // Final lock/unlock to ensure mutex is still functional
    futex_lock(&m);
    futex_unlock(&m);
}

/***********************
 * Reinitialization Test
 ***********************/
void test_reinitialization() {
    futex m;
    for (int i = 0; i < 10; i++) {
        futex_init(&m);
        futex_lock(&m);
        futex_unlock(&m);
    }
}

int main() {
    test_futex_init();
    test_lock_unlock_sequence();
    test_concurrent_access();
    test_mutual_exclusion();
    test_stress();
    test_reinitialization();
    
    printf("All futex tests passed!\n");
    return 0;
}
