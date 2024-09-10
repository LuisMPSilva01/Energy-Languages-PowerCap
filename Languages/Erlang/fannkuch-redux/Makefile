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
	cp fannkuchredux.erlang fannkuchredux.erl
	$(ERLANG_C_PATH) fannkuchredux.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  fannkuchredux main 12" Erlang fannkuch-redux $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  fannkuchredux main 12

mem:
	/usr/bin/time -v $(ERLANG_PATH) -smp enable -noshell -run  fannkuchredux main 12

valgrind:
	valgrind --tool=massif --stacks=yes $(ERLANG_PATH) -smp enable -noshell -run  fannkuchredux main 12

clean:
	rm -f *.beam
	rm -f *.erl