compile:
	cp nbody.erlang-3.erlang nbody.erl
	/opt/erlang/25.2.1/bin/erlc nbody.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000" Erlang n-body

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

mem:
	/usr/bin/time -v  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

clean:
	rm -f *.beam
	rm -f *.erl