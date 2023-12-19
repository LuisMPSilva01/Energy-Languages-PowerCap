NTIMES=1

export NTIMES

compile:
	cp regexredux.fpascal-2.fpascal regexredux.fpascal-2.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux -Fl/usr/lib/x86_64-linux-gnu -oFPASCAL_RUN regexredux.fpascal-2.pas
	mv FPASCAL_RUN regexredux.fpascal-2.fpascal_run
	rm regexredux.fpascal-2.pas
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.fpascal-2.fpascal_run 0 < r../../../inputs/egexredux-input5000000.txt" Pascal regex-redux $(NTIMES)

run:
	./regexredux.fpascal-2.fpascal_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./regexredux.fpascal-2.fpascal_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm *run
	rm *.o
	rm *.ppu
	
valgrind:
	valgrind --tool=massif --stacks=yes ./regexredux.fpascal-2.fpascal_run 0 < ../../../inputs/regexredux-input5000000.txt
