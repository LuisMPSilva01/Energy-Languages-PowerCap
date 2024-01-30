## How Sustainable Is Your Programming language? Analyzing the impact of Power Cap on Energy Efficiency of Programming Languages
### Presented in 1st UMinho Research and Innovation Open Days - University of Minho - Braga - Portugal
  <tr>
    <td align="center">
    <img src="docs/images/header.jpg" alt="header" width="1000" height="200" />
      <p align="center">
        <img src="docs/images/greensoftwarelab.png" alt="greensoftwarelab" width="90" height="90" />
        <img src="docs/images/haslab.png" alt="haslab" width="200" />
      </p>
    </td>
  </tr>

  
### Documents
| Documents         | Links                                                                                               |
|-------------------|-----------------------------------------------------------------------------------------------------|
| Presentation      | [here](https://github.com/LuisMPSilva01/Energy-Languages-PowerCap/blob/master/docs/apresentacao.pdf)         |
| Poster            | [here](https://github.com/LuisMPSilva01/Energy-Languages-PowerCap/blob/master/docs/poster.pdf)                |

### Authors
| Authors       | Links                               |
|---------------|-------------------------------------|
| Simão Cunha   | [GitHub](https://github.com/simaocunha71)    |
| Luís Silva    | [GitHub](https://github.com/LuisMPSilva01)    |
| João Saraiva  | [E-mail](mailto:saraiva@di.uminho.pt) |




### Required libraries
1. RAPL
2. lm-sensors
3. Powercap
4. Raplcap

This libraries can be installed with the following command:

```bash
sudo sh measuresSetup.sh
```

### Setup
In order to install all the required libraries, you should execute the script:

```bash
sudo sh setup.sh
```

Its noteworthy to say that this setup requires manual interaction and its required to use the guide on afterSetup.txt to finish the setup. In a future version this problem will be fixed.

And to generate the input files:

```bash
sudo sh gen-input.sh
```

Then, to generate the CSV file, execute the script:

```bash
sh measurements.sh
```

### Meaning of the CSV file columns

|       Column       |                        Meaning                        |
|:------------------:|:----------------------------------------------------:|
|       **Size**     |   Length of the argument list used by the sorting algorithm;   |
|     **Language**   |               Programming language of the sorting algorithm;              |
|     **Program**    |                     Name of the sorting algorithm;                     |
|     **Package**    | Energy consumption of the entire socket - all cores consumption, GPU, and external core components; |
|      **Core**      |               Energy consumption by all cores and caches;               |
|      **GPU**       |                      Energy consumption by the GPU;                     |
|      **DRAM**      |                      Energy consumption by the RAM;                     |
|      **Time**      |            Algorithm's execution time (in ms);            |
|  **Temperature**   |        Mean temperature in all cores (in ºC);        |
|     **Memory**     |    Total physical memory assigned to the algorithm execution (in KBytes);   |
|   **PowerLimit**   |             Power cap of the cores (in Watts);             |
