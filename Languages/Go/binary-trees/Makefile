NTIMES=1

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o binarytrees.go-2.go_run binarytrees.go-2.go

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