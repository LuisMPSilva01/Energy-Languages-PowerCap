NTIMES=20

export NTIMES

include ../../../config.env

ifndef PHP_PATH
$(error PHP_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PHP_PATH)  fannkuchredux.php-3.php 12" PHP fannkuch-redux $(NTIMES)

run:
	$(PHP_PATH)  fannkuchredux.php-3.php 12

mem:
	/usr/bin/time -v $(PHP_PATH)  fannkuchredux.php-3.php 12

valgrind:
	valgrind --tool=massif --stacks=yes $(PHP_PATH)  fannkuchredux.php-3.php 12
