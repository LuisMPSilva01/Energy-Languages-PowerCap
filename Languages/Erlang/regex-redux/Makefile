NTIMES=1

export NTIMES

compile:
	cp regexredux.erlang-6.erlang regexredux.erl
	/opt/erlang/25.2.1/bin/erlc regexredux.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt" Erlang regex-redux $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.beam
	rm -f *.erl