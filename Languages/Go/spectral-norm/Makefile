NTIMES=1

export NTIMES

compile:
	/opt/src/go1.20/go/bin/go build -o spectralnorm.go-4.go_run spectralnorm.go-4.go

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "./spectralnorm.go-4.go_run 5500" Go spectral-norm $(NTIMES)

run:
	./spectralnorm.go-4.go_run 5500

mem:
	/usr/bin/time -v  ./spectralnorm.go-4.go_run 5500

valgrind:
	valgrind --tool=massif --stacks=yes  ./spectralnorm.go-4.go_run 5500

clean:
	rm -f *.go_run