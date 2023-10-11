compile:
	cp binarytrees.fpascal binarytrees.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux -Mdelphi -oFPASCAL_RUN binarytrees.fpascal-7.pas PasMP.pas
	mv FPASCAL_RUN binarytrees.fpascal_run
	rm binarytrees.pas
	
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./binarytrees.fpascal_run 21" Pascal binary-trees

run:
	./binarytrees.fpascal_run 21

mem:
	/usr/bin/time -v ./binarytrees.fpascal_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.fpascal_run 21
