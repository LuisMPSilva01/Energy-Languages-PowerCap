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
	cp regexredux.erlang-6.erlang regexredux.erl
	$(ERLANG_C_PATH) regexredux.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt" Erlang regex-redux $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  $(ERLANG_PATH) -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  $(ERLANG_PATH) -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.beam
	rm -f *.erl