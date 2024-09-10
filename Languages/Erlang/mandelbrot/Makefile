NTIMES=20

export NTIMES

include ../../../config.env

ifndef ERLANG_PATH
$(error ERLANG_PATH is not set.)
endif

ifndef ERLANG_C_PATH
$(error ERLANG_C_PATH is not set.)
endif


compile:
	cp mandelbrot.erlang-4.erlang mandelbrot.erl
	$(ERLANG_C_PATH) mandelbrot.erl
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(ERLANG_PATH) -smp enable -noshell -run  mandelbrot main 16000" Erlang mandelbrot $(NTIMES)

run:
	$(ERLANG_PATH) -smp enable -noshell -run  mandelbrot main 16000

mem:
	/usr/bin/time -v  $(ERLANG_PATH) -smp enable -noshell -run  mandelbrot main 16000

valgrind:
	valgrind --tool=massif --stacks=yes  $(ERLANG_PATH) -smp enable -noshell -run  mandelbrot main 16000

clean:
	rm -f *.beam
	rm -f *.erl
