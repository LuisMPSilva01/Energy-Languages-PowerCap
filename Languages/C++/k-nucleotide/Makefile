NTIMES=20

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 knucleotide.gpp-2.c++ -o knucleotide.gpp-2.c++.o &&  /usr/bin/g++ knucleotide.gpp-2.c++.o -o knucleotide.gpp-2.gpp_run -lpthread 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.gpp-2.gpp_run 0 < ../../../inputs/knucleotide-input25000000.txt" C++ k-nucleotide $(NTIMES)

run:
	./knucleotide.gpp-2.gpp_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.gpp-2.gpp_run 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.gpp-2.gpp_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *.gpp_run
	rm -f *.o