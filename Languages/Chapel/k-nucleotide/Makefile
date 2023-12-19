NTIMES=1

export NTIMES

compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast knucleotide.chapel-4.chpl -o knucleotide.chapel-4.chapel_run
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.chapel-4.chapel_run --n=0 < ../../../inputs/knucleotide-input25000000.txt" Chapel k-nucleotide $(NTIMES)

run:
	./knucleotide.chapel-4.chapel_run --n=0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.chapel-4.chapel_run --n=0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *_run

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.chapel-4.chapel_run --n=0 < ../../../inputs/knucleotide-input25000000.txt

