compile:
	cp revcomp.erlang revcomp.erl
	/opt/src/otp_src_25.2.1/bin/erlc revcomp.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../revcomp-input25000000.txt" Erlang reverse-complement

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  revcomp main 0 < ../../revcomp-input25000000.txt

clean:
	rm -f *.beam
	rm -f *.erl