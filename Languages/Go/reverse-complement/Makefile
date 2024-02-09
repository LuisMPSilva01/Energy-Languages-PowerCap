NTIMES=20

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o revcomp.go-2.go_run revcomp.go-2.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.go-2.go_run 0 < ../../../inputs/revcomp-input25000000.txt" Go reverse-complement $(NTIMES)

run:
	./revcomp.go-2.go_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v   ./revcomp.go-2.go_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes   ./revcomp.go-2.go_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.go_run