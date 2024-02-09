NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/.rbenv/shims/ruby -W0 fasta.yarv-3.yarv 25000000" Ruby fasta $(NTIMES)

run:
	ruby -W0 fasta.yarv-3.yarv 25000000

mem:
	/usr/bin/time -v  ruby -W0 fasta.yarv-3.yarv 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  ruby -W0 fasta.yarv-3.yarv 25000000
