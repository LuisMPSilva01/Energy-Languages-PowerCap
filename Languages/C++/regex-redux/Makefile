NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 regexredux.gpp-6.c++ -o regexredux.gpp-6.c++.o && $(GPP_PATH) regexredux.gpp-6.c++.o -o regexredux.gpp-6.gpp_run -lpcre2-8 -lpthread 
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.gpp-6.gpp_run 0 < ../../../inputs/regexredux-input5000000.txt" C++ regex-redux $(NTIMES)

run:
	./regexredux.gpp-6.gpp_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.gpp-6.gpp_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.gpp-6.gpp_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f *.gpp_run
	rm -f *.o