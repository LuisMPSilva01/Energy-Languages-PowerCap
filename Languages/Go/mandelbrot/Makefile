NTIMES=1

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o mandelbrot.go-3.go_run mandelbrot.go-3.go

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