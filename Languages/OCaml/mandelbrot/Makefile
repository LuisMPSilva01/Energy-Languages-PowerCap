NTIMES=20

export NTIMES

include ../../../config.env

ifndef OCAMLOP_PATH
$(error OCAMLOP_PATH is not set.)
endif

compile:
	cp mandelbrot.ocaml mandelbrot.ml
	$(OCAMLOP_PATH) -noassert -unsafe -fPIC -nodynlink -inline 100 unix.cmxa mandelbrot.ml -o mandelbrot.ocaml_run
	rm mandelbrot.ml

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./mandelbrot.ocaml_run 16000" OCaml mandelbrot $(NTIMES)

run:
	./mandelbrot.ocaml_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.ocaml_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.ocaml_run 16000

clean:
	rm *.cmi *.cmx *.o *.ocaml_run