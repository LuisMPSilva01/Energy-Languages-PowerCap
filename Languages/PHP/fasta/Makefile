NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "php -n  fasta.php-3.php 25000000" PHP fasta $(NTIMES)

run:
	php -n  fasta.php-3.php 25000000

mem:
	/usr/bin/time -v php -n  fasta.php-3.php 25000000

valgrind:
	valgrind --tool=massif --stacks=yes php -n  fasta.php-3.php 25000000
