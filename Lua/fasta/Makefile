compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  fasta.lua-3.lua 25000000" Lua fasta

run:
	lua  fasta.lua-3.lua 25000000

mem:
	/usr/bin/time -v  lua  fasta.lua-3.lua 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  lua  fasta.lua-3.lua 25000000
