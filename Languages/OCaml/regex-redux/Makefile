compile:
	cp regexredux.ocaml-2.ocaml regexredux.ocaml-2.ml
	ocamlfind ocamlopt -package re -linkpkg -I +str -I +unix -o regexredux.ocaml-2.ocaml_run unix.cmxa str.cmxa regexredux.ocaml-2.ml
	rm regexredux.ocaml-2.ml
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./regexredux.ocaml-2.ocaml_run 0 < ../../inputs/regexredux-input5000000.txt" OCaml regex-redux

run:
	./regexredux.ocaml-2.ocaml_run 0 < ../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  /usr/bin/time -v  /usr/bin/time -v  ./regexredux.ocaml-2.ocaml_run 0 < ../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/time -v  ./regexredux.ocaml-2.ocaml_run 0 < ../../inputs/regexredux-input5000000.txt
