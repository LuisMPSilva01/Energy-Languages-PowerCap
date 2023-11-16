NTIMES=500

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -DSIMD -std=c++11 revcomp.gpp-2.c++ -o revcomp.gpp-2.c++.o && /usr/bin/g++ revcomp.gpp-2.c++.o -o revcomp.gpp-2.gpp_run

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.gpp-2.gpp_run 0 < ../../../inputs/revcomp-input25000000.txt" C++ reverse-complement $(NTIMES)

run:
	./revcomp.gpp-2.gpp_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.gpp-2.gpp_run 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.gpp-2.gpp_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *.gpp_run
	rm -f *.o