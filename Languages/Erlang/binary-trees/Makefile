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
	cp binarytrees.erlang-2.erlang binarytrees.erl
	$(ERLANG_C_PATH) binarytrees.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  binarytrees main 21" Erlang binary-trees $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  binarytrees main 21

mem:
	/usr/bin/time -v  $(ERLANG_PATH) -smp enable -noshell -run  binarytrees main 21

valgrind:
	valgrind --tool=massif --stacks=yes  $(ERLANG_PATH) -smp enable -noshell -run  binarytrees main 21

clean:
	rm -f *.beam
	rm -f *.erl