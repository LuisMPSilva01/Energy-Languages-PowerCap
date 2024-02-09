NTIMES=20

export NTIMES

compile:
	sbcl --load spectralnorm.lisp --eval "(save-lisp-and-die \"spectralnorm.core\" :purify t :toplevel (lambda () (main) (quit)))"

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "sbcl --noinform --core spectralnorm.core 5500" Lisp spectral-norm $(NTIMES)

run:
	sbcl --noinform --core spectralnorm.core 5500

clean:
	rm -rf *.core
	rm -f output

mem:
	/usr/bin/time -v  sbcl --noinform --core spectralnorm.core 5500

valgrind:
	valgrind --tool=massif --stacks=yes  sbcl --noinform --core spectralnorm.core 5500
