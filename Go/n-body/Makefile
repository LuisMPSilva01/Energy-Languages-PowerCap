compile:
	/opt/src/go1.20/go/bin/go build -o nbody.go-3.go_run nbody.go-3.go

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./nbody.go-3.go_run 50000000" Go n-body

run:
	./nbody.go-3.go_run 50000000

mem:
	/usr/bin/time -v ./nbody.go-3.go_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.go-3.go_run 50000000

clean:
	rm -f *.go_run