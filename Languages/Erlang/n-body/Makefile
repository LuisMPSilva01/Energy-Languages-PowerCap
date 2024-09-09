NTIMES=20

export NTIMES

include ../../../config.env

ifndef ERLANG_PATH
$(error ERLANG_PATH is not set.)
endif

ifndef ERLANG_C_PATH
$(error ERLANG_C_PATH is not set.)
endif


compile:
	cp nbody.erlang-3.erlang nbody.erl
	$(ERLANG_C_PATH) nbody.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  nbody main 50000000" Erlang n-body $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  nbody main 50000000

mem:
	/usr/bin/time -v  $(ERLANG_PATH) -smp enable -noshell -run  nbody main 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  $(ERLANG_PATH) -smp enable -noshell -run  nbody main 50000000

clean:
	rm -f *.beam
	rm -f *.erl