NTIMES=2
export NTIMES

problems ?= bspt compress2

measure:
	@for prob in $(problems); do \
		sudo ../../RAPL/main "sudo make -C nofib_repo/real/$$prob/" nofib $$prob $(NTIMES); \
	done

run:
	@for prob in $(problems); do \
		sudo make -C nofib_repo/real/$$prob/; \
	done

clean:
	echo "Cleaning..."

