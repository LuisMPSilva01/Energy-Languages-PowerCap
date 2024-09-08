## Folklore Revisited: Compiling for Speed vs. Compiling for Energy: Using Power Caps to Trade Runtime for Energy Efficiency

| Authors               | Contacts                            |
|-----------------------|-------------------------------------|
| Simão Cunha           | a93262@alunos.uminho.pt             |
| Luís Silva            | pg50564@alunos.uminho.pt            |
| João Saraiva          | saraiva@di.uminho.pt                |
| João Paulo Fernandes  | jpf9731@nyu.edu                     |

### Directory Structure
- **`benchmarks/`**  
  Contains benchmark tests, including:
  - Dacapo
  - Nofib
  - PyPerformance

- **`inputs/`**  
  Required input files for specific problem executions.

- **`Languages/`**  
  Folder with all the languages and problems used for evaluations.

- **`NoteBooks/`**  
  Jupyter notebooks with graphs and calculations used in the paper.

- **`RAPL/`**  
  C program for measuring and limiting CPU power.


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

Note: This setup was not fully tested and my require manual interaction (such as accepting permitions).

2. Generate the input files:

```bash
sudo sh gen-input.sh
```

3. Execute the script to generate the CSV file (this script iterates all the Languages and all of the programs):

```bash
sh measure.sh
```

Note: You might need to update some of the compilers path since we did not change some of the default installation paths from our machine.

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


#### Side note

The setup could not be provided as a docker or virtual machine because the instructions that limit the CPU don't work on that environment.