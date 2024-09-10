NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO fannkuchredux.python3-4.py 12" Python fannkuch-redux $(NTIMES)

run:
	$(PYTHON_PATH) -OO fannkuchredux.python3-4.py 12
mem:
	/usr/bin/time -v $(PYTHON_PATH) -OO fannkuchredux.python3-4.py 12
