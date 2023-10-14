compile:
	export CHPL_GMP=system
	/opt/src/chapel-1.29.0/bin/linux64-x86_64/chpl --fast pidigits.chapel-2.chpl -o pidigits.chapel-2.chapel_run

	
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./pidigits.chapel-2.chapel_run --n=10000" Chapel pidigits

run:
	./pidigits.chapel-2.chapel_run --n=10000

comp2:
	@+if [ "$(CHPL_MAKE_GMP)" != "none" ]; then \
	$(CHPL) -o $@ $(CHPL_FLAGS) $<; \
	else \
	echo "";\
	echo $(CHPL_MAKE_GMP) \
	echo "Can't build pidigits without CHPL_GMP being enabled. See for details:"; \
	echo "http://chapel.cray.com/docs/latest/usingchapel/chplenv.html#chpl-gmp"; \
	echo ""; \
	fi

mem:
	/usr/bin/time -v ./pidigits.chapel-2.chapel_run --n=10000

valgrind:
	valgrind --tool=massif --stacks=yes ./pidigits.chapel-2.chapel_run --n=10000
