NTIMES=20

export NTIMES

include ../../../config.env

ifndef OCAMLOP_PATH
$(error OCAMLOP_PATH is not set.)
endif

compile:
	cp spectralnorm.ocaml-3.ocaml spectralnorm.ocaml-3.ml
	$(OCAMLOP_PATH) -noassert -unsafe -fPIC -nodynlink -I +unix -inline 100 unix.cmxa spectralnorm.ocaml-3.ml -o spectralnorm.ocaml-3.ocaml_run
	rm spectralnorm.ocaml-3.ml


measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.ocaml-3.ocaml_run 5500" OCaml spectral-norm $(NTIMES)

run:
	./spectralnorm.ocaml-3.ocaml_run 5500

mem:
	/usr/bin/time -v ./spectralnorm.ocaml-3.ocaml_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes ./spectralnorm.ocaml-3.ocaml_run 5500

clean:
	rm *.cmi *.cmx *.o *.ocaml_run