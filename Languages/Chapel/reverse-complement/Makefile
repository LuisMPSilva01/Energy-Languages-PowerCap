NTIMES=1

export NTIMES

compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast revcomp.chapel-8.chpl -o revcomp.chapel-8.chapel_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.chapel-8.chapel_run --n=0 < ../../../inputs/revcomp-input25000000.txt" Chapel reverse-complement $(NTIMES)

run:
	./revcomp.chapel-8.chapel_run --n=0 < revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.chapel-8.chapel_run --n=0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *_run

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.chapel-8.chapel_run --n=0 < ../../../inputs/revcomp-input25000000.txt
