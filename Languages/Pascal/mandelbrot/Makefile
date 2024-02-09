NTIMES=20

export NTIMES

compile:
	cp mandelbrot.fpascal-5.fpascal mandelbrot.fpascal-5.pas
	fpc -FuInclude/fpascal -XXs -O3 -Ci- -Cr- -g- -CpCOREAVX -CfAVX -Tlinux  -oFPASCAL_RUN mandelbrot.fpascal-5.pas
	mv FPASCAL_RUN mandelbrot.fpascal-5.fpascal_run
	rm mandelbrot.fpascal-5.pas

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.fpascal-5.fpascal_run 16000" Pascal mandelbrot $(NTIMES)

run:
	./mandelbrot.fpascal-5.fpascal_run 16000

mem:
	/usr/bin/time -v  ./mandelbrot.fpascal-5.fpascal_run 16000
clean:
	rm -f *.o
	rm -f *run
	rm -f *.ppu
valgrind:
	valgrind --tool=massif --stacks=yes  ./mandelbrot.fpascal-5.fpascal_run 16000
