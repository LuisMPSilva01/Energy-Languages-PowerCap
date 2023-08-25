compile:
	cp pidigits.erlang-3.erlang pidigits.erl
	/opt/src/otp_src_25.2.1/bin/erlc pidigits.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  pidigits main 10000" Erlang pidigits

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  pidigits main 10000

mem:
	/usr/bin/time -v  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  pidigits main 10000

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  pidigits main 10000

clean:
	rm -f *.beam
	rm -f *.erl