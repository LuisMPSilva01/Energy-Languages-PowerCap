NTIMES=1

export NTIMES

compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast binarytrees.chapel-3.chpl -o binarytrees.chapel-3.chapel_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.chapel-3.chapel_run --n=21" Chapel binary-trees $(NTIMES)

run:
	./binarytrees.chapel-3.chapel_run --n=21

mem:
	/usr/bin/time -v ./binarytrees.chapel-3.chapel_run --n=21

clean:
	rm -f *_run

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.chapel-3.chapel_run --n=21
