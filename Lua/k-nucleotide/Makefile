compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  knucleotide.lua-2.lua 0 < knucleotide-input25000000.txt" Lua k-nucleotide

run:
	lua  knucleotide.lua-2.lua 0 < knucleotide-input25000000.txt

mem:
	/usr/bin/time -v lua  knucleotide.lua-2.lua 0 < knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes lua  knucleotide.lua-2.lua 0 < knucleotide-input25000000.txt
