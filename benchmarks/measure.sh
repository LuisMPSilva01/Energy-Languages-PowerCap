#!/bin/bash

NTIMES=20

cd ..

# Compile sensors which will be used to calculate cool temperature
cd RAPL || exit
gcc -shared -o sensors.so sensors.c
cd ..

# Update the temperature value
cd Utils
sudo python3 temperatureUpdate.py
cd ..

# Initialize the final measurements CSV
echo "Benchmark,Program,PowerLimit,Package,Core,GPU,DRAM,Time,Temperature,Memory" > benchmarks/measurements_benchmarks.csv

# Lists of programs to run
programs_pyperformance=("2to3" "async_generators" "async_tree" "async_tree_cpu_io_mixed" "async_tree_cpu_io_mixed_tg" "async_tree_eager" "async_tree_eager_cpu_io_mixed" "async_tree_eager_cpu_io_mixed_tg" "async_tree_eager_io" "async_tree_eager_io_tg" "async_tree_eager_memoization" "async_tree_eager_memoization_tg" "async_tree_eager_tg" "async_tree_io" "async_tree_io_tg" "async_tree_memoization" "async_tree_memoization_tg" "async_tree_tg" "asyncio_tcp" "asyncio_tcp_ssl" "asyncio_websockets" "chameleon" "chaos" "comprehensions" "concurrent_imap" "coroutines" "coverage" "crypto_pyaes" "dask" "deepcopy" "deltablue" "django_template" "docutils" "dulwich_log" "fannkuch" "float" "gc_collect" "gc_traversal" "generators" "genshi" "go" "hexiom" "html5lib" "json_dumps" "json_loads" "logging" "mako" "mdp" "meteor_contest" "nbody" "nqueens" "pathlib" "pickle" "pickle_dict" "pickle_list" "pickle_pure_python" "pidigits" "pprint" "pyflate" "python_startup" "python_startup_no_site" "raytrace" "regex_compile" "regex_dna" "regex_effbot" "regex_v8" "richards" "richards_super" "scimark" "spectral_norm" "sqlalchemy_declarative" "sqlalchemy_imperative" "sqlglot" "sqlglot_optimize" "sqlglot_parse" "sqlglot_transpile" "sqlite_synth" "sympy" "telco" "tomli_loads" "tornado_http" "typing_runtime_protocols" "unpack_sequence" "unpickle" "unpickle_list" "unpickle_pure_python" "xml_etree")
programs_nofib=("bspt" "compress2" "fluid" "gg" "hpg" "linear" "parser" "reptile" "smallpt" "anna" "cacheprof" "eff" "fulsom" "grep" "infer" "maillist" "pic" "rsa" "symalg" "ben-raytrace" "compress" "fem" "gamteb" "hidden" "lift" "mkhprog" "prolog" "scs" "veritas")
programs_dacapo=("avrora" "batik" "biojava" "cassandra" "eclipse" "fop" "graphchi" "h2" "h2o" "jme" "jython" "kafka" "luindex" "lusearch" "pmd" "spring" "sunflow" "tomcat" "tradebeans" "tradesoap" "xalan" "zxing")

# Loop over power limit values
for limit in -1 15; do
    pwd
    cd Utils/
    sudo python3 raplCapUpdate.py "$limit" ../RAPL/main.c
    cd ..

    # Make RAPL lib
    cd RAPL/ || exit
    sudo make
    cd ../benchmarks
    
    # Run each Pyperformance program and append results to the final CSV file
    for program in "${programs_pyperformance[@]}"; do
        cd pyperformance || exit
        sudo ../RAPL/main "sudo /usr/local/bin/pyperformance run -b $program" pyperformance "$program" "$NTIMES"
        sudo tail -n +2 measurements.csv >> ../measurements_benchmarks.csv
        cd ..
    done

    # Run each Nofib program and append results to the final CSV file
    for program in "${programs_nofib[@]}"; do
        cd nofib_repo || exit
        sudo ../RAPL/main "sudo make -C nofib/real/$program/" nofib "$program" "$NTIMES"
        sudo tail -n +2 measurements.csv >> ../measurements_benchmarks.csv
        cd ..
    done

    # Run each Dacapo program and append results to the final CSV file
    for program in "${programs_dacapo[@]}"; do
        cd dacapo || exit
        sudo ../RAPL/main "sudo /usr/lib/jvm/jdk-20/bin/java -jar dacapo-23.11-chopin.jar $program" dacapo "$program" "$NTIMES"        
        sudo tail -n +2 measurements.csv >> ../measurements_benchmarks.csv
        cd ..
    done
    cd ..
done

cd RAPL/ || exit
sudo make clean
cd ..

sudo reboot
