NTIMES=1

export NTIMES

compile:
	sbcl --eval "(ql:quickload 'cl-ppcre)" --load regexredux.lisp --eval "(save-lisp-and-die \"regexredux.core\" :purify t :toplevel (lambda () (main) (quit)))"

run:
	sbcl --noinform --core regexredux.core 0 < ../../../inputs/regexredux-input5000000.txt

measure:
	sudo modprobe msr
	#sudo ../../../RAPL/main "sbcl --noinform --core regexredux.core 0 < ../../../inputs/regexredux-input5000000.txt" Lisp regex-redux $(NTIMES)

clean:
	rm -rf *core
	rm -f output

mem:
	/usr/bin/time -v regexredux.core 0 < ../../../inputs/regexredux-input5000000.txt Lisp regex-redux

valgrind:
	valgrind --tool=massif --stacks=yes sbcl --noinform --core regexredux.core 0 < ../../../inputs/regexredux-input5000000.txt Lisp regex-redux
