compile:
	/opt/src/go1.20/go/bin/go build -o knucleotide.go-7.go_run knucleotide.go-7.go

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./knucleotide.go-7.go_run  0 < ../../inputs/knucleotide-input25000000.txt" Go k-nucleotide

run:
	./knucleotide.go-7.go_run  0 < ../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  ./knucleotide.go-7.go_run  0 < ../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  ./knucleotide.go-7.go_run  0 < ../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.go_run