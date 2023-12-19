NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "php -d memory_limit=1024M knucleotide.php-4.php 0 < ../../../inputs/knucleotide-input25000000.txt" PHP k-nucleotide $(NTIMES)

run:
	php -d memory_limit=1024M knucleotide.php-4.php 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v php -d memory_limit=1024M knucleotide.php-4.php 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes php -d memory_limit=1024M knucleotide.php-4.php 0 < ../../../inputs/knucleotide-input25000000.txt
