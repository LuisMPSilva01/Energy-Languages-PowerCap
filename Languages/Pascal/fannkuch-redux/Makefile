NTIMES=1

export NTIMES

compile:
	cp fannkuchredux.fpascal fannkuchredux.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux  -oFPASCAL_RUN fannkuchredux.pas
	mv FPASCAL_RUN fannkuchredux.fpascal_run
	rm fannkuchredux.pas
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.fpascal_run 12" Pascal fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.fpascal_run 12

mem:
	/usr/bin/time -v ./fannkuchredux.fpascal_run 12
clean:
	rm -f *.o
	rm -f *run
	rm -f *.ppu
	
valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.fpascal_run 12

