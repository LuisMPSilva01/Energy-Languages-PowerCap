NTIMES=20

export NTIMES

compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge -fopenmp -D_FILE_OFFSET_BITS=64 -I/usr/include/apr-1.0 binarytrees.gcc-3.c -o binarytrees.gcc-3.gcc_run -lapr-1 -lgomp -lm
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.gcc-3.gcc_run 21" C binary-trees $(NTIMES)

run:
	./binarytrees.gcc-3.gcc_run 21

mem:
	/usr/bin/time -v ./binarytrees.gcc-3.gcc_run 21

clean:
	rm -f *.gcc_run
	rm -f out