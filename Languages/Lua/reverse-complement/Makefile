NTIMES=20

export NTIMES

compile:

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "lua  revcomp.lua-2.lua 0 < ../../../inputs/revcomp-input25000000.txt" Lua reverse-complement $(NTIMES)

run:
	lua  revcomp.lua-2.lua 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v lua  revcomp.lua-2.lua 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes lua  revcomp.lua-2.lua 0 < ../../../inputs/revcomp-input25000000.txt
