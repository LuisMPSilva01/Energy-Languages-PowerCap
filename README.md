### Authors
* [Simão Cunha](https://github.com/simaocunha71)
* [Luís Silva](https://github.com/LuisMPSilva01) 


### Required libraries
1. RAPL
2. lm-sensors
3. Powercap
4. Raplcap

This libraries can be installed with
```sudo sh measuresSetup.sh```

### Setup
In order to install all the required libraries, you should execute the script:

```sudo sh setup.sh```

Its noteworthy to say that this setup requires manual interaction and its required to use the guide on afterSetup.txt to finish the setup. In a future version this problem will be fixed.

And to generate the input files:

```sudo sh gen-input.sh```

Then, to generate the CSV file, execute the script:

```sh measurements.sh```

### Meaning of the CSV file columns
* **Size** : Length of the argument list used by the sorting algorithm;
* **Language** : Programming language of the sorting algorithm;
* **Program** : Name of the sorting algorithm;
* **Package** : Energy consumption of the entire socket- all cores consumption, GPU e external core components);
* **Core** : Energy consumption by all cores and caches;
* **GPU** : Energy consumption by the GPU;
* **DRAM** : Energy consumption by the RAM;
* **Time** : Algorithm's execution time (in ms);
* **Temperature** : Mean temperature in all cores (in ºC);
* **Memory** : Total physical memory assigned to the algorithm execution (in KBytes);
* **Cost** : Development cost (in $);
* **PowerLimit** : Power cap of the cores (in Watts)