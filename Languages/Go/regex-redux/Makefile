NTIMES=1

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o regexredux.go-5.go_run regexredux.go-5.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.go-5.go_run 0 < ../../../inputs/regexredux-input5000000.txt" Go regex-redux $(NTIMES)

run:
	./regexredux.go-5.go_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.go-5.go_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.go-5.go_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.go_run