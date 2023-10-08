compile:
	cp nbody.erlang-3.erlang nbody.erl
	/opt/src/otp_src_25.2.1/bin/erlc nbody.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000" Erlang n-body

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

mem:
	/usr/bin/time -v  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  nbody main 50000000

clean:
	rm -f *.beam
	rm -f *.erl