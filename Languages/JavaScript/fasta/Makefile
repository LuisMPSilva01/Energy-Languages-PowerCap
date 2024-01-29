NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.nvm/versions/node/v19.4.0/bin/node fasta.node-4.js 25000000" JavaScript fasta $(NTIMES)

run:
	node fasta.node-4.js 25000000

mem:
	/usr/bin/time -v node fasta.node-4.js 25000000


valgrind:
	valgrind --tool=massif --stacks=yes node fasta.node-4.js 25000000
