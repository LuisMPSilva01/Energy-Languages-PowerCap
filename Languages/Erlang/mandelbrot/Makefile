NTIMES=1

export NTIMES

compile:
	cp mandelbrot.erlang-4.erlang mandelbrot.erl
	/opt/erlang/25.2.1/bin/erlc mandelbrot.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  mandelbrot main 16000" Erlang mandelbrot $(NTIMES)

run:
	/opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  mandelbrot main 16000

mem:
	/usr/bin/time -v  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  mandelbrot main 16000

valgrind:
	valgrind --tool=massif --stacks=yes  /opt/erlang/25.2.1/bin/erl -smp enable -noshell -run  mandelbrot main 16000

clean:
	rm -f *.beam
	rm -f *.erl
