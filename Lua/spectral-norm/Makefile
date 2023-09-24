compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  spectralnorm.lua 5500" Lua spectral-norm

run:
	lua  spectralnorm.lua 5500

mem:
	/usr/bin/time -v lua  spectralnorm.lua 5500

valgrind:
	valgrind --tool=massif --stacks=yes lua  spectralnorm.lua 5500
