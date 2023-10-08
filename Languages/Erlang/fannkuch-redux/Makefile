compile:
	cp fannkuchredux.erlang fannkuchredux.erl
	/opt/src/otp_src_25.2.1/bin/erlc fannkuchredux.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12" Erlang fannkuch-redux

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

mem:
	/usr/bin/time -v /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

valgrind:
	valgrind --tool=massif --stacks=yes /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fannkuchredux main 12

clean:
	rm -f *.beam
	rm -f *.erl