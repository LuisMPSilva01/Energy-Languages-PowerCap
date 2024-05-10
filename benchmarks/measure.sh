#!/bin/bash

NTIMES=2
#NTIMES=1

# Compile sensors which will be used to calculate cool temperature
cd ../RAPL || exit
gcc -shared -o sensors.so sensors.c
cd ..

# Update the temperature value
cd Utils/
sudo python3 temperatureUpdate.py
cd ..

echo "Benchmark,Program,PowerLimit,Package,Core,GPU,DRAM,Time,Temperature,Memory" > measurements_benchmarks.csv

# Loop over power limit values
for limit in 10 25; do
    cd Utils/
    sudo python3 raplCapUpdate.py "$limit" ../RAPL/main.c
    cd ..

    # Make RAPL lib
    cd RAPL/ || exit
    #sudo rm sensors.so
    sudo make
    cd ..

    cd benchmarks/dacapo || exit
    for problem in avrora; do
        sudo ../../RAPL/main "sudo /usr/lib/jvm/jdk-20/bin/java -jar dacapo-23.11-chopin.jar $problem" dacapo $problem "$NTIMES"
        # Specify the input file name
        file="measurements_benchmarks.csv"
        sudo tail -n +2 "$file" >>measurements_benchmarks.csv
    done
    cd ../..

    #cd benchmarks/nofib || exit
    #for problem in bspt compress2; do
    #    sudo ../../RAPL/main "sudo make -C nofib_repo/real/$problem/" nofib "$problem" "$NTIMES"
    #    # Specify the input file name
    #    file="measurements_benchmarks.csv"
    #    sudo tail -n +2 "$file" >>measurements_benchmarks.csv
    #done

    #cd ../..

    #cd benchmarks/pyperformance || exit
    #for problem in 2to3 async_generators; do
    #    sudo ../../RAPL/main "sudo ~/.local/bin/pyperformance run -b $problem" pyperformance "$problem" "$NTIMES"
    #    # Specify the input file name
    #    file="measurements_benchmarks.csv"
    #    sudo tail -n +2 "$file" >>measurements_benchmarks.csv
    #done
    #cd ../..

done

cd RAPL/ || exit
sudo make clean
cd ..

#sudo reboot

