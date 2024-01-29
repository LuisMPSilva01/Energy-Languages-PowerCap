NTIMES=1

export NTIMES

compile:
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast spectralnorm.chapel-2.chpl -o spectralnorm.chapel-2.chapel_run
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.chapel-2.chapel_run --n=5500" Chapel spectral-norm $(NTIMES)

run:
	./spectralnorm.chapel-2.chapel_run --n=5500

mem:
	/usr/bin/time -v ./spectralnorm.chapel-2.chapel_run --n=5500

clean:
	rm -f *_run

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.chapel-2.chapel_run --n=5500
