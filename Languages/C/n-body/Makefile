NTIMES=5

export NTIMES

compile:
	/usr/bin/gcc -pipe -Wall -O3 -fomit-frame-pointer -march=ivybridge  nbody.gcc-9.c -o nbody.gcc-9.gcc_run 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.gcc-9.gcc_run 50000000" C n-body $(NTIMES)

run:
	./nbody.gcc-9.gcc_run 50000000

mem:
	/usr/bin/time -v ./nbody.gcc-9.gcc_run 50000000

clean:
	rm -f *.gcc_run