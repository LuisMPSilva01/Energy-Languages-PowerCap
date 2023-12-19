NTIMES=1

export NTIMES

compile:
	ifort -O3 -fast -openmp knucleotide.f90 -o knucleotide.ifc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.ifc_run 0 < ../../../inputs/knucleotide-input25000000.txt" Fortran k-nucleotide $(NTIMES)

run:
	./knucleotide.ifc_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v  ./knucleotide.ifc_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f *ifc_run
	
	
valgrind:
	valgrind --tool=massif --stacks=yes  ./knucleotide.ifc_run 0 < ../../../inputs/knucleotide-input25000000.txt
