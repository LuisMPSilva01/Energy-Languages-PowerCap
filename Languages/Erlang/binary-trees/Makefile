NTIMES=20

export NTIMES

compile:
	cp binarytrees.erlang-2.erlang binarytrees.erl
	/opt/erlang/25.2.1/bin/erlc binarytrees.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  binarytrees main 21" Erlang binary-trees $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  binarytrees main 21

mem:
	/usr/bin/time -v  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  binarytrees main 21

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  binarytrees main 21

clean:
	rm -f *.beam
	rm -f *.erl