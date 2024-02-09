NTIMES=20

export NTIMES

compile:
	cp regexredux.ocaml-3.ocaml regexredux.ocaml-3.ml
	~/.opam/5.0.0/bin/ocamlopt -noassert -unsafe -fPIC -nodynlink -inline 100 -O3 -I ~/.opam/5.0.0/lib/re unix.cmxa re.cmxa -ccopt -march=ivybridge regexredux.ocaml-3.ml -o regexredux.ocaml-3.ocaml_run 
	rm regexredux.ocaml-2.ml
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./regexredux.ocaml-3.ocaml_run 0 < ../../../inputs/regexredux-input5000000.txt" OCaml regex-redux $(NTIMES)

run:
	./regexredux.ocaml-3.ocaml_run 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v  /usr/bin/time -v  /usr/bin/time -v  ./regexredux.ocaml-3.ocaml_run 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/time -v  ./regexredux.ocaml-3.ocaml_run 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm *.cmi *.cmx *.o *.ocaml_run
