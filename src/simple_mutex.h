#pragma once

#include <stdatomic.h>

typedef atomic_char32_t futex;

void futex_lock(futex *mutex);
void futex_unlock(futex *mutex);
void futex_init(futex *mutex);
