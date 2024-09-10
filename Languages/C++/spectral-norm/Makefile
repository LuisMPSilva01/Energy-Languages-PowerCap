NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -fopenmp spectralnorm.gpp-6.c++ -o spectralnorm.gpp-6.c++.o && $(GPP_PATH) spectralnorm.gpp-6.c++.o -o spectralnorm.gpp-6.gpp_run -fopenmp 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.gpp-6.gpp_run 5500" C++ spectral-norm $(NTIMES)

run:
	./spectralnorm.gpp-6.gpp_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.gpp-6.gpp_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.gpp-6.gpp_run 5500

clean:
	rm -f *.gpp_run
	rm -f *.o