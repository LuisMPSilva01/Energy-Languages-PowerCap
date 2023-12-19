NTIMES=1

export NTIMES

compile:
	cp fannkuchredux.erlang fannkuchredux.erl
	/opt/erlang/25.2.1/bin/erlc fannkuchredux.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12" Erlang fannkuch-redux $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

mem:
	/usr/bin/time -v /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

valgrind:
	valgrind --tool=massif --stacks=yes /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

clean:
	rm -f *.beam
	rm -f *.erl