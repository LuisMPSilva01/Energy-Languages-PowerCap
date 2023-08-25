compile:
	cp regexredux.erlang-6.erlang regexredux.erl
	/opt/src/otp_src_25.2.1/bin/erlc regexredux.erl
xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../regexredux-input50000.txt" Erlang regex-redux

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../regexredux-input50000.txt

mem:
	/usr/bin/time -v  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../regexredux-input50000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run -noinput -run regexredux main 0 < ../../regexredux-input50000.txt

clean:
	rm -f *.beam
	rm -f *.erl