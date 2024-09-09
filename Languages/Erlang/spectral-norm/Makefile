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
	cp spectralnorm.erlang-2.erlang spectralnorm.erl
	$(ERLANG_C_PATH) spectralnorm.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  spectralnorm main 5500" Erlang spectral-norm $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  spectralnorm main 5500

mem:
	/usr/bin/time -v $(ERLANG_PATH) -smp enable -noshell -run  spectralnorm main 5500

valgrind:
	valgrind --tool=massif --stacks=yes $(ERLANG_PATH) -smp enable -noshell -run  spectralnorm main 5500

clean:
	rm -f *.beam
	rm -f *.erl