WARNINGS := -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \
            -Wwrite-strings -Wmissing-prototypes -Wmissing-declarations \
            -Wredundant-decls -Wnested-externs -Winline -Wno-long-long \
            -Wstrict-prototypes -Werror=odr -Werror=lto-type-mismatch -Werror=strict-aliasing -Wstrict-overflow=2
			#-Wconversion

CFLAGS+=-lpthread -pthread $(WARNINGS) -Isrc/ --std=gnu2x
LDFLAGS+=-lpthread -pthread ${WARNINGS}
#--static

RELEASE_FLAGS=-O3 -march=native -mtune=native -flto -fwhole-program -fomit-frame-pointer
# -fopenmp -fomit-frame-pointer
DEBUG_FLAGS=-ggdb -g3 -O0 -fno-inline -march=native

SOURCES=
SOURCES+=test-lib.c
SOURCES+=lib-graph-exec.c
OBJECTS+=$(patsubst %.c,build/%.o,$(SOURCES))


.PHONY: all main main-prog debug help clean mpropper

all: main 

-include build/*.d

main: CFLAGS+=$(RELEASE_FLAGS)
main: LDFLAGS+=-s $(RELEASE_FLAGS)
main: main_prog

main_prog: test-lib

debug: CFLAGS+=$(DEBUG_FLAGS)
debug: LDFLAGS+=$(DEBUG_FLAGS)
debug: main_prog

help:
	@echo "Makefile help"
	@echo "    make main          Compiles and links the program (stripped)"
	@echo "    make debug         Compiles and links the program with debugging flags"
	@echo "    make clean         Cleans compilation temporary files (slow down subsequent ones)"
	@echo "    make mpropper      Cleans compilation files and executable and compiled files"
	@echo "    make help          Displays this help"
	

test-lib: $(OBJECTS)
	$(CC) $^ $(LDFLAGS) -o test-lib
	@echo "La compilation a réussie! Le programme s'apelle test-lib, lancez le en terminal avec la commande ./test-lib"

build:
	@mkdir -p build/


build/%.o: src/%.c Makefile build
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

clean:
	-rm -rf build

mpropper: clean
	-rm test_lib
