NTIMES=1

export NTIMES

compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast nbody.chapel-3.chpl -o nbody.chapel-3.chapel_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.chapel-3.chapel_run--n=50000000" Chapel n-body $(NTIMES)

run:
	./nbody.chapel-3.chapel_run --n=50000000

mem:
	/usr/bin/time -v ./nbody.chapel-3.chapel_run--n=50000000

clean:
	rm -f *_run

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.chapel-3.chapel_run--n=50000000
