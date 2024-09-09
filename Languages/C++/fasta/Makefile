NTIMES=20

export NTIMES

include ../../../config.env

ifndef GPP_PATH
$(error GPP_PATH is not set.)
endif

compile:
	$(GPP_PATH) -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 fasta.gpp-8.c++ -o fasta.gpp-8.c++.o && $(GPP_PATH) fasta.gpp-8.c++.o -o fasta.gpp-8.gpp_run -lpthread 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.gpp-8.gpp_run 25000000" C++ fasta $(NTIMES)

run:
	./fasta.gpp-8.gpp_run 25000000

mem:
	/usr/bin/time -v ./fasta.gpp-8.gpp_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.gpp-8.gpp_run 25000000

clean:
	rm -f *.gpp_run
	rm -f *.o