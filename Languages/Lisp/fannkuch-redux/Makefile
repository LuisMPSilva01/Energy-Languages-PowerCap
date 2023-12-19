NTIMES=1

export NTIMES

compile:
	sbcl --load fannkuch.lisp --eval "(save-lisp-and-die \"fannkuch.core\" :purify t :toplevel (lambda () (main) (quit)))"

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "sbcl --noinform --core fannkuch.core 12" Lisp fannkuch-redux $(NTIMES)

run:
	sbcl --noinform --core fannkuch.core 12

clean:
	rm -rf *core
	rm -f output

mem:
	/usr/bin/time -v sbcl --noinform --core fannkuch.core 12


valgrind:
	valgrind --tool=massif --stacks=yes sbcl --noinform --core fannkuch.core 12

