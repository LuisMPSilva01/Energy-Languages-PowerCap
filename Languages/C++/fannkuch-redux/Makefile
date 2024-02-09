NTIMES=20

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 -fopenmp fannkuchredux.gpp-6.c++ -o fannkuchredux.gpp-6.c++.o && /usr/bin/g++ fannkuchredux.gpp-6.c++.o -o fannkuchredux.gpp-6.gpp_run -fopenmp 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.gpp-6.gpp_run 12" C++ fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.gpp-6.gpp_run 12

mem:
	 /usr/bin/time -v  ./fannkuchredux.gpp-6.gpp_run 12
valgrind:
	valgrind --tool=massif --stacks=yes  ./fannkuchredux.gpp-6.gpp_run 12

clean:
	rm -f *.gpp_run
	rm -f *.o