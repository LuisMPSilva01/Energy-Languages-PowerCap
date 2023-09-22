compile:
	cp spectralnorm.python3-5.python3 spectralnorm.python3-5.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO spectralnorm.python3-5.py 5500" Python spectral-norm

run:
	python3.11 -OO spectralnorm.python3-5.py 5500

mem:
	/usr/bin/time -v python3.11 -OO spectralnorm.python3-5.py 5500
