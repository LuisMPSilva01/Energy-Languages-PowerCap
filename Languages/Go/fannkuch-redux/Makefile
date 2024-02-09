NTIMES=20

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o fannkuchredux.go-3.go_run fannkuchredux.go-3.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.go-3.go_run 12" Go fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.go-3.go_run 12

mem:
	/usr/bin/time -v ./fannkuchredux.go-3.go_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.go-3.go_run 12

clean:
	rm -f *.go_run