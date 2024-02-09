NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "php  fannkuchredux.php-3.php 12" PHP fannkuch-redux $(NTIMES)

run:
	php  fannkuchredux.php-3.php 12

mem:
	/usr/bin/time -v php  fannkuchredux.php-3.php 12

valgrind:
	valgrind --tool=massif --stacks=yes php  fannkuchredux.php-3.php 12
