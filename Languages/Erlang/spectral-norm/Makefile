NTIMES=20

export NTIMES

compile:
	cp spectralnorm.erlang-2.erlang spectralnorm.erl
	/opt/erlang/25.2.1/bin/erlc spectralnorm.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500" Erlang spectral-norm $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

mem:
	/usr/bin/time -v /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

valgrind:
	valgrind --tool=massif --stacks=yes /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

clean:
	rm -f *.beam
	rm -f *.erl