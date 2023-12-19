NTIMES=1

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "lua  fasta.lua-3.lua 25000000" Lua fasta $(NTIMES)

run:
	lua  fasta.lua-3.lua 25000000

mem:
	/usr/bin/time -v  lua  fasta.lua-3.lua 25000000

valgrind:
	valgrind --tool=massif --stacks=yes  lua  fasta.lua-3.lua 25000000
