NTIMES=20

export NTIMES

include ../../../config.env

ifndef LUA_PATH
$(error LUA_PATH is not set.)
endif

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(LUA_PATH)  mandelbrot.lua-6.lua 16000" Lua mandelbrot $(NTIMES)

run:
	$(LUA_PATH)  mandelbrot.lua-6.lua 16000

mem:
	/usr/bin/time -v  $(LUA_PATH)  mandelbrot.lua-6.lua 16000

valgrind:
	valgrind --tool=massif --stacks=yes  $(LUA_PATH)  mandelbrot.lua-6.lua 16000
