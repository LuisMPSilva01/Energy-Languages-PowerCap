NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "lua  binarytrees.lua-2.lua 21" Lua binary-trees $(NTIMES)

run:
	lua  binarytrees.lua-2.lua 21

mem:
	/usr/bin/time -v lua  binarytrees.lua-2.lua 21

valgrind:
	valgrind --tool=massif --stacks=yes lua  binarytrees.lua-2.lua 21
