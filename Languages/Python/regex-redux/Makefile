NTIMES=20

export NTIMES

include ../../../config.env

ifndef PYTHON_PATH
$(error PYTHON_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PYTHON_PATH) -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt" Python regex-redux $(NTIMES)

run:
	$(PYTHON_PATH) -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v $(PYTHON_PATH) -OO regexredux.py 0 < ../../../inputs/regexredux-input5000000.txt
