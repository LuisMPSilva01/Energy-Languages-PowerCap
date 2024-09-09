NTIMES=20

export NTIMES

include ../../../config.env

ifndef PHP_PATH
$(error PHP_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PHP_PATH) -n  fasta.php-3.php 25000000" PHP fasta $(NTIMES)

run:
	$(PHP_PATH) -n  fasta.php-3.php 25000000

mem:
	/usr/bin/time -v $(PHP_PATH) -n  fasta.php-3.php 25000000

valgrind:
	valgrind --tool=massif --stacks=yes $(PHP_PATH) -n  fasta.php-3.php 25000000
