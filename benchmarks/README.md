# Language-Specific Benchmarks Artifact for "Trading Runtime for Energy Efficiency: Leveraging Power Caps to Save Energy Across Programming Languages"

## Overview

In addition to benchmarks from the **Computer Language Benchmarks Game (CLBG)**, we have included language-specific benchmarks. The chosen languages represent different paradigms and execution models:

- **Haskell**: Functional, compiled language
- **Python**: Interpreted, multi-paradigm language
- **Java**: Object-oriented language running on a virtual machine

### Benchmarks and Versions

| Language              | Benchmark                                                                                           | Version                                               |
|:----------------------:|:----------------------------------------------------------------------------------------------------:|:-----------------------------------------------------:|
| Python                | [pyperformance](https://pyperformance.readthedocs.io)                                    | 1.11.0                                                |
| Java                  | [Dacapo](https://www.dacapobench.org/)                                                               | 23.11-chopin                                          |
| Haskell               | [nofib](https://gitlab.haskell.org/ghc/nofib/-/tree/7ffecc8115865fea9995a951091e6ff23cf8ca3a)       | 7ffecc8115865fea9995a951091e6ff23cf8ca3a (last commit SHA) |


## Setup Instructions

1. **Download Language-Specific Benchmarks**

    First, download the language-specific benchmarks and set up the `benchmarks/` folder by running:

```bash
bash setup_benchmarks.sh
```

2. **Verify File Structure**

    After running the setup script, you should have the following directory structure:

```
benchmarks/
├── dacapo/
│   ├── dacapo-23.11-chopin.zip
│   └── dacapo-23.11-chopin/
│       └── (dacapo-23.11-chopin repository files)
├── nofib_repo/
│   ├── nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a.zip
│   └── nofib/
│       └── (nofib repository files)
└── pyperformance/
    └── (no files in this folder yet)
```

3. **Run Benchmarks**

    To configure the benchmarks and execute them, use:

```bash
bash measure.sh
```

4. **Results**
   
    The execution will generate a file named `measurements_benchmarks.csv`. This file will have the same columns as the `../measurements.csv` from the CLBG problems results.
