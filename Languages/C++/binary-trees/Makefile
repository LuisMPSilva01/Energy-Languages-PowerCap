NTIMES=500

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=gnu++17 binarytrees.gpp-7.c++ -o binarytrees.gpp-7.c++.o && /usr/bin/g++ binarytrees.gpp-7.c++.o -o binarytrees.gpp-7.gpp_run -ltbb 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.gpp-7.gpp_run 21" C++ binary-trees $(NTIMES)

run:
	./binarytrees.gpp-7.gpp_run 21

mem:
	/usr/bin/time -v ./binarytrees.gpp-7.gpp_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.gpp-7.gpp_run 21

clean:
	rm -f *.gpp_run
	rm -f *.o