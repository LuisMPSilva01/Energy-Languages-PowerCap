main : main.c rapl.o sensors.o
	gcc -O2  -o main main.c rapl.o sensors.o $(shell pkg-config --libs --static raplcap-powercap) -lm
	chmod 777 main

rapl.o : rapl.c rapl.h
	gcc -O2  -c rapl.c  -lm

sensors.o : sensors.c sensors.h
	gcc -O2  -c sensors.c  -lm

clean:
	rm -rf main main.o rapl.o sensors.o