NTIMES=1

export NTIMES

compile:
	cp fasta.fpascal-4.fpascal fasta.fpascal-4.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux -Sh -oFPASCAL_RUN  fasta.fpascal-4.pas
	mv FPASCAL_RUN fasta.fpascal-4.fpascal_run
	rm fasta.fpascal-4.pas

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.fpascal-4.fpascal_run 25000000" Pascal fasta $(NTIMES)

run:
	./fasta.fpascal-4.fpascal_run 25000000

mem:
	/usr/bin/time -v  ./fasta.fpascal-4.fpascal_run 25000000
clean:
	rm -f *.o
	rm -f *run
	rm -f *.ppu
valgrind:
	valgrind --tool=massif --stacks=yes  ./fasta.fpascal-4.fpascal_run 25000000
