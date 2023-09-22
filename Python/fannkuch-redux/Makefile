compile:
	cp fannkuchredux.python3-4.python3 fannkuchredux.python3-4.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO fannkuchredux.python3-4.py 12" Python fannkuch-redux

run:
	python3.11 -OO fannkuchredux.python3-4.py 12
mem:
	/usr/bin/time -v python3.11 -OO fannkuchredux.python3-4.py 12
