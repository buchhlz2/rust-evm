# rust-evm

## A very rudimentary implementation of EVM in Rust

### Prerequistes

- Install [`solc`](https://docs.soliditylang.org/en/v0.8.7/installing-solidity.html) and [`rust`](https://www.rust-lang.org/tools/install)

### Usage

- Compile & run `decompile.rs` with 1 argument (the binary file) to read & print contract bytecode:
  ```
  rustc --out-dir ./bin decompile.rs
  ./bin/decompile <path_to_binary_file>
  ```
- Example using contract `contract.sol` & generated binary file `Addition.bin-runtime` are provided
  - Get binary manually by running:
    ```
    solc --bin-runtime --optimize -o ./bin contract.sol --overwrite
    ```
- Read & print opcodes:
  ```
  ./bin/decompile ./bin/Addition.bin-runtime
  ```
