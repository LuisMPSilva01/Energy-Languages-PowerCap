NTIMES=20

export NTIMES

include ../../../config.env

ifndef OCAMLOP_PATH
$(error OCAMLOP_PATH is not set.)
endif

compile:
	cp knucleotide.ocaml-3.ocaml knucleotide.ocaml-3.ml
	$(OCAMLOP_PATH) -noassert -unsafe -fPIC -nodynlink -inline 100 unix.cmxa knucleotide.ocaml-3.ml -o knucleotide.ocaml-3.ocaml_run
	rm knucleotide.ocaml-3.ml

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./knucleotide.ocaml-3.ocaml_run 0 < ../../../inputs/knucleotide-input25000000.txt" OCaml k-nucleotide $(NTIMES)

run:
	./knucleotide.ocaml-3.ocaml_run 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v ./knucleotide.ocaml-3.ocaml_run 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./knucleotide.ocaml-3.ocaml_run 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm *.cmi *.cmx *.o *.ocaml_run