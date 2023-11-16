NTIMES=500

export NTIMES

compile:
	/usr/bin/g++ -c -pipe -O3 -fomit-frame-pointer -march=ivybridge  -std=c++17 fasta.gpp-8.c++ -o fasta.gpp-8.c++.o && /usr/bin/g++ fasta.gpp-8.c++.o -o fasta.gpp-8.gpp_run -lpthread 

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