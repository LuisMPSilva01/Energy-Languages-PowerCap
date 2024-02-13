NTIMES=20

export NTIMES

compile:
	python3 dotfileGenerator.py 
measure:
	sudo ../../RAPL/main "dot -Ksfdp -o output.png -Tpng file.dot"  C Graphviz $(NTIMES)

run:
	dot -Ksfdp -o output.png -Tpng file.dot

clean:
	rm file.dot
	rm output.png