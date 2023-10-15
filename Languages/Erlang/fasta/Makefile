compile:
	cp fasta.erlang-2.erlang fasta.erl
	/opt/erlang/25.2.1/bin/erlc fasta.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000" Erlang fasta

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000

mem:
	/usr/bin/time -v /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000
valgrind:
	valgrind --tool=massif --stacks=yes /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000

clean:
	rm -f *.beam
	rm -f *.erl