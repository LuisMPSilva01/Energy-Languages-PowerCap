NTIMES=20

export NTIMES

include ../../../config.env

ifndef GO_PATH
$(error GO_PATH is not set.)
endif

compile:
	$(GO_PATH) build -o nbody.go-3.go_run nbody.go-3.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.go-3.go_run 50000000" Go n-body $(NTIMES)

run:
	./nbody.go-3.go_run 50000000

mem:
	/usr/bin/time -v ./nbody.go-3.go_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.go-3.go_run 50000000

clean:
	rm -f *.go_run