NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO nbody.py 50000000" Python n-body $(NTIMES)

run:
	$(PYTHON_PATH) -OO nbody.py 50000000

mem:
	/usr/bin/time -v $(PYTHON_PATH) -OO nbody.py 50000000
