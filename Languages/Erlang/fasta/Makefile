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
	cp fasta.erlang-2.erlang fasta.erl
	$(ERLANG_C_PATH) fasta.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  fasta main 25000000" Erlang fasta $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  fasta main 25000000

mem:
	/usr/bin/time -v $(ERLANG_PATH) -smp enable -noshell -run  fasta main 25000000
valgrind:
	valgrind --tool=massif --stacks=yes $(ERLANG_PATH) -smp enable -noshell -run  fasta main 25000000

clean:
	rm -f *.beam
	rm -f *.erl