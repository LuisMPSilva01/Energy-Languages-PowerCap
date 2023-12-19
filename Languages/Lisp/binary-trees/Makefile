NTIMES=1

export NTIMES

compile:
	sbcl --eval "(ql:quickload 'sb-concurrency)" --eval "(load \"binarytrees.lisp\")" --eval "(save-lisp-and-die \"binarytrees.core\" :purify t :toplevel (lambda () (main) (quit)))"

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "sbcl --noinform --core binarytrees.core 21" Lisp binary-trees $(NTIMES)

run:
	sbcl --noinform --core binarytrees.core 21

clean:
	rm -rf *.core
	rm -f output

mem:
	/usr/bin/time -v sbcl --noinform --core binarytrees.core 21

valgrind:
	valgrind --tool=massif --stacks=yes sbcl --noinform --core binarytrees.core 21
