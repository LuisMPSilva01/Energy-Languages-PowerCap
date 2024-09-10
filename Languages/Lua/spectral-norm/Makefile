NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  spectralnorm.lua 5500" Lua spectral-norm $(NTIMES)

run:
	$(LUA_PATH)  spectralnorm.lua 5500

mem:
	/usr/bin/time -v $(LUA_PATH)  spectralnorm.lua 5500

valgrind:
	valgrind --tool=massif --stacks=yes $(LUA_PATH)  spectralnorm.lua 5500
