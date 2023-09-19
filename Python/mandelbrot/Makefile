compile:
	cp mandelbrot.python3-7.python3 mandelbrot.python3-7.py
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python3.11 -OO mandelbrot.python3-7.py 16000" Python mandelbrot

run:
	python3.11 -OO mandelbrot.python3-7.py 16000

mem:
	/usr/bin/time -v python3.11 -OO mandelbrot.python3-7.py 16000
