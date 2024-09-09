NTIMES=20

export NTIMES

include ../../../config.env

ifndef GO_PATH
$(error GO_PATH is not set.)
endif

compile:
	$(GO_PATH) build -o fasta.go-2.go_run fasta.go-2.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.go-2.go_run 25000000" Go fasta $(NTIMES)

run:
	./fasta.go-2.go_run 25000000

mem:
	/usr/bin/time -v ./fasta.go-2.go_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.go-2.go_run 25000000

clean:
	rm -f *.go_run