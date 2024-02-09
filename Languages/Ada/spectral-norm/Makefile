NTIMES=20

export NTIMES

compile:
	gnatchop -r -w spectralnorm.gnat-4.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f spectralnorm.adb -o spectralnorm.gnat-4.gnat_run 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.gnat-4.gnat_run 5500" Ada spectral-norm $(NTIMES)

run:
	./spectralnorm.gnat-4.gnat_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.gnat-4.gnat_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.gnat-4.gnat_run 5500	

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o
	rm -f output