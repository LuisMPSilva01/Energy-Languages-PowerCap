compile:
	gnatchop -r -w mandelbrot.gnat-3.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f mandelbrot.adb -o mandelbrot.gnat-3.gnat_run

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