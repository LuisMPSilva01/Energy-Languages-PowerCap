compile:
	cp knucleotide.erlang-3.erlang knucleotide.erl
	/opt/src/otp_src_25.2.1/bin/erlc knucleotide.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < knucleotide-input25000000.txt" Erlang k-nucleotide

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  knucleotide main 0 < knucleotide-input25000000.txt

clean:
	rm -f *.beam
	rm -f *.erl