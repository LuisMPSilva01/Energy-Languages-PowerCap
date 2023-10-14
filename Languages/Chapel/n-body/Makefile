compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast nbody.chapel-3.chpl -o nbody.chapel-3.chapel_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./nbody.chapel-3.chapel_run--n=50000000" Chapel n-body

run:
	./nbody.chapel-3.chapel_run --n=50000000

mem:
	/usr/bin/time -v ./nbody.chapel-3.chapel_run--n=50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.chapel-3.chapel_run--n=50000000
