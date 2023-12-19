NTIMES=1

export NTIMES

compile:
	ifort -O3 -fast -opt-streaming-stores always fasta.ifc-4.f90 -o fasta.ifc-4.ifc_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.ifc-4.ifc_run 25000000" Fortran fasta $(NTIMES)

run:
	./fasta.ifc-4.ifc_run 25000000

mem:
	/usr/bin/time -v ./fasta.ifc-4.ifc_run 25000000

clean:
	rm -f *ifc_run
	
	
valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.ifc-4.ifc_run 25000000
