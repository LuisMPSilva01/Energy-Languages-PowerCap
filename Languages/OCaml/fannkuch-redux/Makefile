NTIMES=20

export NTIMES

include ../../../config.env

ifndef OCAMLOP_PATH
$(error OCAMLOP_PATH is not set.)
endif

compile:
	cp fannkuchredux.ocaml-4.ocaml fannkuchredux.ocaml-4.ml
	$(OCAMLOP_PATH) -noassert -unsafe -fPIC -nodynlink -inline 100 unix.cmxa fannkuchredux.ocaml-4.ml -o fannkuchredux.ocaml-4.ocaml_run
	rm fannkuchredux.ocaml-4.ml

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./fannkuchredux.ocaml-4.ocaml_run 12" OCaml fannkuch-redux $(NTIMES)

run:
	./fannkuchredux.ocaml-4.ocaml_run 12

mem:
	/usr/bin/time -v ./fannkuchredux.ocaml-4.ocaml_run 12

valgrind:
	valgrind --tool=massif --stacks=yes ./fannkuchredux.ocaml-4.ocaml_run 12

clean:
	rm *.cmi *.cmx *.o *.ocaml_run