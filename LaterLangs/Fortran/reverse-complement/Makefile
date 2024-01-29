NTIMES=1

export NTIMES

compile:
	ifort -O3 -fast revcomp.f90 -o revcomp.ifc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./revcomp.ifc_run 0 < ../../../inputs/revcomp-input25000000.txt" Fortran reverse-complement $(NTIMES)

run:
	./revcomp.ifc_run 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./revcomp.ifc_run 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f *ifc_run
	
valgrind:
	valgrind --tool=massif --stacks=yes ./revcomp.ifc_run 0 < ../../../inputs/revcomp-input25000000.txt
