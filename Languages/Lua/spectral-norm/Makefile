NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "lua  spectralnorm.lua 5500" Lua spectral-norm $(NTIMES)

run:
	lua  spectralnorm.lua 5500

mem:
	/usr/bin/time -v lua  spectralnorm.lua 5500

valgrind:
	valgrind --tool=massif --stacks=yes lua  spectralnorm.lua 5500
