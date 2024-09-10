NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 nbody.gpp-0.c++ -o nbody.gpp-0.c++.o &&  $(GPP_PATH) nbody.gpp-0.c++.o -o nbody.gpp-0.gpp_run
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