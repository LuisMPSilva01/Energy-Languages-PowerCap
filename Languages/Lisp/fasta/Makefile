NTIMES=20

export NTIMES

compile:
	sbcl --load fasta.lisp --eval "(save-lisp-and-die \"fasta.core\" :purify t :toplevel (lambda () (main) (quit)))"

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "sbcl --noinform --core fasta.core 25000000" Lisp fasta $(NTIMES)

run:
	sbcl --noinform --core fasta.core 25000000

clean:
	rm -rf *core
	rm -f output

mem:
	/usr/bin/time -v sbcl --noinform --core fasta.core 25000000

valgrind:
	valgrind --tool=massif --stacks=yes sbcl --noinform --core fasta.core 25000000
