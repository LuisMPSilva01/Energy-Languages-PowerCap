NTIMES=20

export NTIMES

include ../../../config.env

ifndef GO_PATH
$(error GO_PATH is not set.)
endif

compile:
	$(GO_PATH) build -o mandelbrot.go-3.go_run mandelbrot.go-3.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.go-3.go_run 16000" Go mandelbrot $(NTIMES)

run:
	./mandelbrot.go-3.go_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.go-3.go_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.go-3.go_run 16000

clean:
	rm -f *.go_run