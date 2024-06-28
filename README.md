## Folklore Revisited: Compiling for Speed vs. Compiling for Energy: Using Power Caps to Trade Runtime for Energy Efficiency

| Authors               | Contacts                            |
|-----------------------|-------------------------------------|
| Simão Cunha           | simaopscunha@outlook.pt             |
| Luís Silva            | luis.m.peixoto.s@gmail.com          |
| João Saraiva          | saraiva@di.uminho.pt                |
| João Paulo Fernandes  | jpf9731@nyu.edu                     |

### Required Libraries
1. RAPL
2. lm-sensors
3. Powercap
4. Raplcap

These libraries can be installed with the following command:

```bash
sudo sh measuresSetup.sh
```

### Setup
1. To install all the required libraries, execute the script:

```bash
sudo sh setup.sh
```

Note that this setup requires manual interaction. Follow the guide in afterSetup.txt to complete the setup. This issue will be addressed in a future version.

2. Generate the input files:

```bash
sudo sh gen-input.sh
```

3. Execute the script to generate the CSV file:

```bash
sh measurements.sh
```

### Meaning of the CSV file columns

|      Column      |                        Meaning                                                                     |
|:----------------:|:--------------------------------------------------------------------------------------------------:|
|      **Size**    | Length of the argument list used by the sorting algorithm                                          |
|    **Language**  | Programming language of the sorting algorithm                                                      |
|    **Program**   | Name of the sorting algorithm                                                                      |
|    **Package**   | Energy consumption of the entire socket - all cores consumption, GPU, and external core components |
|     **Core**     | Energy consumption by all cores and caches                                                         |
|     **GPU**      | Energy consumption by the GPU                                                                      |
|     **DRAM**     | Energy consumption by the RAM                                                                      |
|     **Time**     | Algorithm's execution time (in ms)                                                                 |
| **Temperature**  | Mean temperature in all cores (in Celsius degrees)                                                 |
|    **Memory**    | Total physical memory assigned to the algorithm execution (in KBytes)                              |
|  **PowerLimit**  | Power cap of the cores (in Watts)                                                                  |
