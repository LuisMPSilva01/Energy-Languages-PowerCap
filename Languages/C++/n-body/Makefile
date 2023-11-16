NTIMES=500

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 nbody.gpp-0.c++ -o nbody.gpp-0.c++.o &&  /usr/bin/g++ nbody.gpp-0.c++.o -o nbody.gpp-0.gpp_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.gpp-0.gpp_run 50000000" C++ n-body $(NTIMES)

run:
	./nbody.gpp-0.gpp_run 50000000

mem:
	/usr/bin/time -v ./nbody.gpp-0.gpp_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes ./nbody.gpp-0.gpp_run 50000000

clean:
	rm -f *.gpp_run
	rm -f *.o