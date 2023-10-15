compile:
	cp pidigits.ocaml-3.ocaml pidigits.ocaml-3.ml
	ocamlfind ocamlopt -o pidigits.ocaml-3.ocaml_run -I +unix -I /usr/local/lib/ocaml/gmp -package gmp,zarith -linkpkg -noassert -unsafe -fPIC -nodynlink -inline 100 unix.cmxa gmp.cmxa pidigits.ocaml-3.ml
#	ocamlfind ocamlopt -o pidigits.ocaml-3.ocaml_run -I +unix -I /usr/local/lib/ocaml/gmp -package gmp -linkpkg -noassert -unsafe -fPIC -nodynlink -inline 100 unix.cmxa gmp.cmxa pidigits.ocaml-3.ml
	rm pidigits.ocaml-3.ml

xmeasure:
	sudo modprobe msr
	sudo ../../RAPL/main "./pidigits.ocaml-3.ocaml_run 2000" OCaml pidigits

run:
	./pidigits.ocaml-3.ocaml_run 2000

mem:
	/usr/bin/time -v ../pidigits.ocaml-3.ocaml_run 2000

valgrind:
	valgrind --tool=massif --stacks=yes ./pidigits.ocaml-3.ocaml_run 2000

clean:
	rm *.cmi *.cmx *.o *.ocaml_run