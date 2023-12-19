NTIMES=1

export NTIMES

compile:
	cp nbody.ocaml nbody.ml
	ocamlopt -noassert -unsafe -fPIC -nodynlink -inline 100  nbody.ml -o nbody.ocaml_run
	rm nbody.ml
measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./nbody.ocaml_run 50000000" OCaml n-body $(NTIMES)

run:
	./nbody.ocaml_run 50000000

mem:
	/usr/bin/time -v  ./nbody.ocaml_run 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  ./nbody.ocaml_run 50000000

clean:
	rm *.cmi *.cmx *.o *.ocaml_run