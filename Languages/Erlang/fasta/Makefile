compile:
	cp fasta.erlang-2.erlang fasta.erl
	/opt/src/otp_src_25.2.1/bin/erlc fasta.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000" Erlang fasta

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000

mem:
	/usr/bin/time -v /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000
valgrind:
	valgrind --tool=massif --stacks=yes /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  fasta main 25000000

clean:
	rm -f *.beam
	rm -f *.erl