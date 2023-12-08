compile:
	cp nbody.fpascal nbody.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux  -oFPASCAL_RUN nbody.pas
	mv FPASCAL_RUN nbody.fpascal_run
	rm nbody.pas

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./nbody.fpascal_run 50000000" Pascal n-body

run:
	./nbody.fpascal_run 50000000

mem:
	/usr/bin/time -v  ./nbody.fpascal_run 50000000
clean:
	rm *run
	rm *.o
	rm *.ppu
	
valgrind:
	valgrind --tool=massif --stacks=yes  ./nbody.fpascal_run 50000000
