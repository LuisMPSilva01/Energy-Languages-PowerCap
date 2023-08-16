compile:
	raco make binarytrees.racket-3.racket
	
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/binarytrees.racket-3_racket.zo 21" Racket binary-trees

run:
	racket ./compiled/binarytrees.racket-3_racket.zo 21

mem:
	/usr/bin/time -v  racket ./compiled/binarytrees.racket-3_racket.zo 21

valgrind:
	valgrind --tool=massif --stacks=yes  racket ./compiled/binarytrees.racket-3_racket.zo 21

clean:
	rm -rf compiled/
