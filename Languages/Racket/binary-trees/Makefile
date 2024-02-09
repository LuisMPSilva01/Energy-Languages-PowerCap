NTIMES=20

export NTIMES

compile:
	/home/diguest/bin/raco make binarytrees.racket-3.racket
	
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/home/diguest/bin/racket ./compiled/binarytrees.racket-3_racket.zo 21" Racket binary-trees $(NTIMES)

run:
	racket ./compiled/binarytrees.racket-3_racket.zo 21

mem:
	/usr/bin/time -v  racket ./compiled/binarytrees.racket-3_racket.zo 21

valgrind:
	valgrind --tool=massif --stacks=yes  racket ./compiled/binarytrees.racket-3_racket.zo 21

clean:
	rm -rf compiled/
