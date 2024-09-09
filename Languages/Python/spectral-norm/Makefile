NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO spectralnorm.python3-5.py 5500" Python spectral-norm $(NTIMES)

run:
	$(PYTHON_PATH) -OO spectralnorm.python3-5.py 5500

mem:
	/usr/bin/time -v $(PYTHON_PATH) -OO spectralnorm.python3-5.py 5500
