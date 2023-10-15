compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 pidigits.yarv-5.yarv 10000" Ruby pidigits

run:
	ruby -W0 pidigits.yarv-5.yarv 10000

mem:
	/usr/bin/time -v  ruby -W0 pidigits.yarv-5.yarv 10000

valgrind:
	valgrind --tool=massif --stacks=yes  ruby -W0 pidigits.yarv-5.yarv 10000
