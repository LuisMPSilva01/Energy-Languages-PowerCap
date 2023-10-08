compile:
	gnatchop -r -w spectralnorm.gnat-4.gnat
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f spectralnorm.adb -o spectralnorm.gnat-4.gnat_run 
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp spectralnorm.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp division.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp division-d.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp division-s.ads
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp spectral_utils.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp spectral_utils-d.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp spectral_utils-dist.adb
	x86_64-linux-gnu-gcc-11 -c -O3 -fomit-frame-pointer -march=ivybridge -gnatNp spectral_utils-s.adb
	x86_64-linux-gnu-gnatbind-11 -x spectralnorm.ali
	x86_64-linux-gnu-gnatlink-11 spectralnorm.ali -O3 -fomit-frame-pointer -march=ivybridge -o spectralnorm.gnat-4.gnat_run


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./spectralnorm.gnat-4.gnat_run 5500" Ada spectral-norm

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