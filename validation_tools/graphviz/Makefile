NTIMES=20

export NTIMES

compile:

measure:
	sudo ../../RAPL/main "dot -Ksfdp -o output.png -Tpng file.dot"  C Graphviz $(NTIMES)

run:
	dot -Ksfdp -o output.png -Tpng file.dot

clean:
	rm output.png