NTIMES=20

export NTIMES

include ../../../config.env

ifndef GO_PATH
$(error GO_PATH is not set.)
endif

compile:
	$(GO_PATH) build -o pidigits.go-4.go_run pidigits.go-4.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./pidigits.go-4.go_run 10000" Go pidigits $(NTIMES)

run:
	./pidigits.go-4.go_run 10000

mem:
	/usr/bin/time -v ./pidigits.go-4.go_run 10000

valgrind:
	valgrind --tool=massif --stacks=yes ./pidigits.go-4.go_run 10000

clean:
	rm -f *.go_run