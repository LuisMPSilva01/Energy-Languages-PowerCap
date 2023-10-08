#!/bin/bash
NTIMES=20

#Compile sensors wich will be used to calculate cool temperature
cd RAPL
gcc -shared -o sensors.so sensors.c
cd ..

#Update the temperature value
cd Utils/
python3 temperatureUpdate.py

#Update the number of times the program will run on each case TODO PRECISO ATUALIZAR ISTO PARA TODOS OS PROGRAMAS
python3 ntimesUpdate.py $NTIMES ../Languages/C/fannkuch-redux/Makefile
cd ..
      
echo "Language, Program, PowerLimit, Package , Core(s) , GPU , DRAM , Time (ms) , Temperature , Memory" > measurements.csv

# Loop over power limit values
for limit in 0 1 5 10 20 30 40 50 60 70 80 90 100 120 140 160 180 200 250 300 350 400 450 500 600 700 800 900 1000 1200 1400 1600 1800 2000
    do
    cd Utils/
    python3 raplCapUpdate.py $limit ../RAPL/main.c
    cd ..
    #Make RAPL lib
    cd RAPL/
    rm sensors.so
    make
    cd ..

<<comment
    for language in "Languages"/*; do
        for program in "$language"/*; do
            cd $program
            make measure 

            # Specify the input file name
            file="measurements.csv"
            tail -n +2 "$file" >> ../../../measurements.csv;
            make clean
            cd ../../..
        done
    done
comment
    for program in "Languages/C"/*; do
        cd $program
        make measure 

        # Specify the input file name
        file="measurements.csv"
        tail -n +2 "$file" >> ../../../measurements.csv;
        make clean
        cd ../../..
    done
done

cd RAPL/
make clean
cd ..