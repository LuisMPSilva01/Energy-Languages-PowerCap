## Trading Runtime for Energy Efficiency: Leveraging Power Caps to Save Energy Across Programming Languages
In Proceedings of the 17th ACM SIGPLAN International Conference on Software Language Engineering (SLE '24).
https://doi.org/10.1145/3687997.3695638

| Authors               | Contacts                            |
|:---------------------:|:-----------------------------------:|
| Simão Cunha           | a93262@alunos.uminho.pt             |
| Luís Silva            | pg50564@alunos.uminho.pt            |
| João Saraiva          | saraiva@di.uminho.pt                |
| João Paulo Fernandes  | jpf9731@nyu.edu                     |

### Requirements
- Debian-based Linux distributions (needs to work with RAPLCap)
- Intel processor
- Non containerized environment

### Directory Structure
- **`benchmarks/`**  
  Contains benchmark tests (see benchmarks/README.md for more details), including:
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
sudo sh raplLibrariesSetup.sh
```

### Setup
1. To install all the required language compilers, interpreters and libraries, execute the script:

```bash
sudo sh languagesSetup.sh
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

Note: You might need to update some of the compilers path since we did not change some of the default installation paths from our machine (the paths are defined on config.env).

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

#### Cite Us

```
@inproceedings{10.1145/3687997.3695638,
author = {Cunha, Sim\~{a}o and Silva, Lu\'{\i}s and Saraiva, Jo\~{a}o and Fernandes, Jo\~{a}o Paulo},
title = {Trading Runtime for Energy Efficiency: Leveraging Power Caps to Save Energy across Programming Languages},
year = {2024},
isbn = {9798400711800},
publisher = {Association for Computing Machinery},
address = {New York, NY, USA},
url = {https://doi.org/10.1145/3687997.3695638},
doi = {10.1145/3687997.3695638},
abstract = {Energy efficiency of software is crucial in minimizing environmental impact and reducing operational costs of ICT systems. Energy efficiency is therefore a key area of contemporary software language engineering research. A recurrent discussion that excites our community is whether runtime performance is always a proxy for energy efficiency. While a generalized intuition seems to suggest this is the case, this intuition does not align with the fact that energy is the accumulation of power over time; hence, time is only one of the factors in this accumulation. We focus on the other factor, power, and the impact that capping it has on the energy efficiency of running software.
We conduct an extensive investigation comparing regular and power-capped executions of 9 benchmark programs obtained from The Computer Language Benchmarks Game, across 20 distinct programming languages. Our results show that employing power caps can be used to trade running time, which is degraded, for energy efficiency, which is improved, in all the programming languages and in all benchmarks that were considered. We observe overall energy savings of almost 14\% across the 20 programming languages, with notable savings of 27\% in Haskell. This saving, however, comes at the cost of an overall increase of the program's execution time of 91\% in average.
We are also able to draw similar observations using language specific benchmarks for programming languages of different paradigms and with different execution models. This is achieved analyzing a wide range of benchmark programs from the nofib Benchmark Suite of Haskell Programs, DaCapo Benchmark Suite for Java, and the Python Performance Benchmark Suite. We observe energy savings of approximately 8\% to 21\% across the test suites, with execution time increases ranging from 21\% to 46\%. Notably, the DaCapo suite exhibits the most significant values, with 20.84\% energy savings and a 45.58\% increase in execution time.
Our results have the potential to drive significant energy savings in the context of computational tasks for which runtime is not critical, including Batch Processing Systems, Background Data Processing and Automated Backups.},
booktitle = {Proceedings of the 17th ACM SIGPLAN International Conference on Software Language Engineering},
pages = {130–142},
numpages = {13},
keywords = {Energy Efficiency, Green Software, Language Benchmarking, Power Cap, Programming Languages},
location = {Pasadena, CA, USA},
series = {SLE '24}
}
```
