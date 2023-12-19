NTIMES=1

export NTIMES

compile:
	sbcl --load knucleotide.lisp --eval "(save-lisp-and-die \"knucleotide.core\" :purify t :toplevel (lambda () (main) (quit)))"

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "sbcl --noinform --core knucleotide.core 5500" Lisp k-nucleotide $(NTIMES)

run:
	sbcl --noinform --core knucleotide.core --userinit /dev/null --load knucleotide.sbcl-6.sbcl_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -rf *core
	rm -f output

mem:
	/usr/bin/time -v  sbcl --noinform --core knucleotide.core --userinit /dev/null --load knucleotide.sbcl-6.sbcl_run 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  sbcl --noinform --core knucleotide.core --userinit /dev/null --load knucleotide.sbcl-6.sbcl_run 0 < ../../../inputs/knucleotide-input25000000.txt
