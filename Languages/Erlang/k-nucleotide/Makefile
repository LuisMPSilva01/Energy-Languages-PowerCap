NTIMES=1

export NTIMES

compile:
	cp knucleotide.erlang-3.erlang knucleotide.erl
	/opt/erlang/25.2.1/bin/erlc knucleotide.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < ../../../inputs/knucleotide-input25000000.txt" Erlang k-nucleotide $(NTIMES)
run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.beam
	rm -f *.erl