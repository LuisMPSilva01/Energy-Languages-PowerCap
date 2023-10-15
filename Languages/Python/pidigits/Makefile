compile:
	cp pidigits.python3-2.python3 pidigits.python3-2.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "/usr/local/bin/python3.11 -OO pidigits.python3-2.py 10000" Python pidigits

run:
	/usr/local/bin/python3.11 -OO pidigits.python3-2.py 10000
mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO pidigits.python3-2.py 10000
