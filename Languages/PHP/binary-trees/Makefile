NTIMES=20

export NTIMES

include ../../../config.env

ifndef PHP_PATH
$(error PHP_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(PHP_PATH) -n -d memory_limit=4096M binarytrees.php-5.php 21" PHP binary-trees $(NTIMES)

run:
	$(PHP_PATH) -n -d memory_limit=4096M binarytrees.php-5.php 21

mem:
	/usr/bin/time -v $(PHP_PATH) -n -d memory_limit=4096M binarytrees.php-5.php 21

valgrind:
	valgrind --tool=massif --stacks=yes $(PHP_PATH) -n -d memory_limit=4096M binarytrees.php-5.php 21
