NTIMES=20

export NTIMES

include ../../../config.env

ifndef PHP_PATH
$(error PHP_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PHP_PATH) -n -d memory_limit=1024M revcomp.php-3.php 0 < ../../../inputs/revcomp-input25000000.txt" PHP reverse-complement $(NTIMES)

run:
	$(PHP_PATH) -n -d memory_limit=1024M revcomp.php-3.php 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v $(PHP_PATH) -n -d memory_limit=1024M revcomp.php-3.php 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(PHP_PATH) -n -d memory_limit=1024M revcomp.php-3.php 0 < ../../../inputs/revcomp-input25000000.txt



