compile:
	gnatchop -r -w mandelbrot.gnat-3.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f mandelbrot.adb -o mandelbrot.gnat-3.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp mandelbrot.adb
	x86_64-linux-gnu-gnatbind-11 -x mandelbrot.ali
	x86_64-linux-gnu-gnatlink-11 mandelbrot.ali -O3 -fomit-frame-pointer -march=ivybridge -o mandelbrot.gnat-3.gnat_run
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./mandelbrot.gnat-3.gnat_run 16000" Ada mandelbrot

run:
	./mandelbrot.gnat-3.gnat_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.gnat-3.gnat_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.gnat-3.gnat_run 16000

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o