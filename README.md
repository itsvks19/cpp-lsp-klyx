# C++ LSP Server Klyx Extension

This extension uses the [clangd](https://clangd.llvm.org/) language server to provide comprehensive language support for C++ in [Klyx](https://github.com/klyx-dev/klyx).

## Installation

Before installing this extension, install `clangd`:

```bash
sudo apt install clangd
```

Verify that `clangd` is available:

```bash
clangd --version
```

## Project Setup

For optimal results, ensure your C++ project has a `compile_commands.json` file. Generate it using:

```bash
# For CMake projects
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON .

# For Makefile projects (requires bear)
bear -- make
```
