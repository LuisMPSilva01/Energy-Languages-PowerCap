compile:
	cp spectralnorm.erlang-2.erlang spectralnorm.erl
	/opt/src/otp_src_25.2.1/bin/erlc spectralnorm.erl
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500" Erlang spectral-norm

run:
	/opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

mem:
	/usr/bin/time -v /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

valgrind:
	valgrind --tool=massif --stacks=yes /opt/src/otp_src_25.2.1/bin/erl -smp enable -noshell -run  spectralnorm main 5500

clean:
	rm -f *.beam
	rm -f *.erl