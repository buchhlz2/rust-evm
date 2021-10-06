# rust-evm-practice

## A very rudimentary implementation of EVM in Rust
Note -- purely for personal learning & doesn't work in an accurate or functional capacity

### Prerequistes

- Install [`solc`](https://docs.soliditylang.org/en/v0.8.7/installing-solidity.html) and [`rust`](https://www.rust-lang.org/tools/install)

### Usage

- Compile & run `main.rs` with 2 arguments (either "run" or "debug", and the binary file path) to read & print contract bytecode:
  ```
  cargo build
  cargo run run <path_to_file>
  ...
  cargo run debug <path_to_file>
  ```
- Example using contract `contract.sol` & generated binary file `Addition.bin-runtime` are provided
  - Get binary manually by running:
    ```
    solc --bin-runtime --optimize -o ./bin contract.sol --overwrite
    ```
