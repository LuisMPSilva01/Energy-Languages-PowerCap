NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt" JavaScript reverse-complement $(NTIMES)

run:
	node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes node revcomp.node-7.js 0 < ../../../inputs/revcomp-input25000000.txt
