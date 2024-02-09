NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "php -n -d memory_limit=4096M binarytrees.php-5.php 21" PHP binary-trees $(NTIMES)

run:
	php -n -d memory_limit=4096M binarytrees.php-5.php 21

mem:
	/usr/bin/time -v php -n -d memory_limit=4096M binarytrees.php-5.php 21

valgrind:
	valgrind --tool=massif --stacks=yes php -n -d memory_limit=4096M binarytrees.php-5.php 21
