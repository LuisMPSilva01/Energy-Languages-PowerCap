NTIMES=20

export NTIMES

include ../../../config.env

ifndef NODE_PATH
$(error NODE_PATH is not set.)
endif


compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(NODE_PATH) fasta.node-4.js 25000000" JavaScript fasta $(NTIMES)

run:
	node fasta.node-4.js 25000000

mem:
	/usr/bin/time -v node fasta.node-4.js 25000000


valgrind:
	valgrind --tool=massif --stacks=yes node fasta.node-4.js 25000000
