#include "lib-threaded-tomatsulo.h"

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
