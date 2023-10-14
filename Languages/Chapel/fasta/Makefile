compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast fasta.chapel-6.chpl -o fasta.chapel-6.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fasta.chapel-6.chapel_run --n=25000000" Chapel fasta

run:
	./fasta.chapel-6.chapel_run --n=25000000

mem:
	/usr/bin/time -v ./fasta.chapel-6.chapel_run --n=25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.chapel-6.chapel_run --n=25000000
