NTIMES=20

export NTIMES

include ../../../config.env

ifndef GNATCHOP_PATH
$(error GNATCHOP_PATH is not set.)
endif

compile:
	$(GNATCHOP_PATH) -r -w fasta.gnat-5.gnat
	PATH=/usr/local/gcc-fsf-gnat/bin:$$PATH && \
	export PATH && \
	LD_LIBRARY_PATH=/usr/local/gcc-fsf-gnat/lib:$$LD_LIBRARY_PATH && \
	export LD_LIBRARY_PATH && \
	gnatmake -O3 -fomit-frame-pointer -march=ivybridge -gnatNp -f fasta.adb -o fasta.gnat-5.gnat_run


measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fasta.gnat-5.gnat_run 25000000" Ada fasta $(NTIMES)

run:
	./fasta.gnat-5.gnat_run 25000000

mem:
	/usr/bin/time -v ./fasta.gnat-5.gnat_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.gnat-5.gnat_run 25000000

clean:
	rm -f *.gnat_run 
	rm -f *.adb
	rm -f *.ads
	rm -f *.ali
	rm -f *.o
	rm -f output