NTIMES=1

export NTIMES

compile:
	cp binarytrees.fpascal binarytrees.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux -Mdelphi -oFPASCAL_RUN binarytrees.pas
	mv FPASCAL_RUN binarytrees.fpascal_run
	rm binarytrees.pas
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./binarytrees.fpascal_run 21" Pascal binary-trees $(NTIMES)

run:
	./binarytrees.fpascal_run 21

mem:
	/usr/bin/time -v ./binarytrees.fpascal_run 21

clean:
	rm -f *.o
	rm -f *run
	rm -f *.ppu
valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.fpascal_run 21
