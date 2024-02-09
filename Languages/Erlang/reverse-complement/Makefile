NTIMES=20

export NTIMES

compile:
	cp revcomp.erlang revcomp.erl
	/opt/erlang/25.2.1/bin/erlc revcomp.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../../inputs/revcomp-input25000000.txt" Erlang reverse-complement $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.beam
	rm -f *.erl