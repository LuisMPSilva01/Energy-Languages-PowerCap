NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO spectralnorm.python3-5.py 5500" Python spectral-norm $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO spectralnorm.python3-5.py 5500

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO spectralnorm.python3-5.py 5500
