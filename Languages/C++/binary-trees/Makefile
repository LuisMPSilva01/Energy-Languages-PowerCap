NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=gnu++17 binarytrees.gpp-7.c++ -o binarytrees.gpp-7.c++.o && $(GPP_PATH) binarytrees.gpp-7.c++.o -o binarytrees.gpp-7.gpp_run -ltbb 
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