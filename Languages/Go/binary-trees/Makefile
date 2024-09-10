NTIMES=20

export NTIMES

include ../../../config.env

ifndef GO_PATH
$(error GO_PATH is not set.)
endif

compile:
	$(GO_PATH) build -o binarytrees.go-2.go_run binarytrees.go-2.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.go-2.go_run 21" Go binary-trees $(NTIMES)

run:
	./binarytrees.go-2.go_run 21

mem:
	/usr/bin/time -v ./binarytrees.go-2.go_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.go-2.go_run 21

clean:
	rm -f *.go_run