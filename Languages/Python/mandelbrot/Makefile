NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/local/bin/python3.11 -OO mandelbrot.python3-7.py 16000" Python mandelbrot $(NTIMES)

run:
	/usr/local/bin/python3.11 -OO mandelbrot.python3-7.py 16000

mem:
	/usr/bin/time -v /usr/local/bin/python3.11 -OO mandelbrot.python3-7.py 16000
